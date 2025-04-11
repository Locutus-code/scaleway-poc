use random_string;

use redis::AsyncCommands;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json;
use rocket::{State, get, post};

use lib::{MessageType, SqsPayload};

use crate::helpers::log_message::request_counter;
use crate::models::state::ApplicationState;
use crate::models::url::{ResponseStatus, UrlGetResponse, UrlPostRequest, UrlPostResponse};
use crate::workers::redis::get_conn;
use crate::workers::sqs::enqueue_msg;

fn random_key() -> String {
    random_string::generate(8, random_string::charsets::ALPHA)
}

#[post("/url", format = "json", data = "<payload>")]
pub async fn post_url<'a>(
    payload: Json<UrlPostRequest>,
    state: &State<ApplicationState>,
) -> Json<UrlPostResponse> {
    request_counter(state);
    let mut conn = get_conn(state).await;
    let key = random_key();
    // let db_result = conn.set(key.as_str(), &payload.url).await.unwrap();
    let _db_result = redis::cmd("SET")
        .arg(&[key.as_str(), &payload.url])
        .exec_async(&mut conn)
        .await;

    match _db_result {
        Ok(_db_result) => rocket::info!("DB Save succeeded: {}", key),
        Err(e) => rocket::error!("Db Save failed: {}", e),
    }

    let response = UrlPostResponse::new(
        state,
        ResponseStatus::Accepted,
        key.clone(),
        payload.url.clone(),
    );

    enqueue_msg(
        state,
        serde_json::to_string(&SqsPayload {
            msg_type: MessageType::Url,
            msg: &key,
            request_id: response.request_id,
            service_id: state.service_id,
        })
        .unwrap()
        .as_str(),
    )
    .await;

    Json(response)
}

#[get("/url/<key>")]
pub async fn get_url<'a>(key: &'a str, state: &State<ApplicationState>) -> Json<UrlGetResponse> {
    request_counter(state);
    let mut conn = get_conn(state).await;
    let result = conn.get(key).await;
    let response = match result {
        Ok(result) => UrlGetResponse {
            short_url: Some(key.to_string()),
            long_url: result,
            status: ResponseStatus::Done,
        },
        Err(e) => {
            rocket::error!("key not found: {}, error: {}", key, e);
            UrlGetResponse {
                short_url: Some(key.to_string()),
                long_url: None,
                status: ResponseStatus::Error,
            }
        }
    };
    Json(response)
}
