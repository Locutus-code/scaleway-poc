use rocket::serde::json::Json;
use rocket::{State, get, post};

use random_string;

use crate::models::state::ApplicationState;
use crate::models::url::{ResponseStatus, UrlGetResponse, UrlPostResponse};
use crate::workers::redis::get_conn;
use redis::AsyncCommands;

fn random_key() -> String {
    random_string::generate(8, random_string::charsets::ALPHA)
}

#[post("/url/<url>")]
pub async fn post_url<'a>(url: &'a str, state: &State<ApplicationState>) -> Json<UrlPostResponse> {
    let mut conn = get_conn(state).await;
    let key = random_key();
    let _: () = conn.set(key.as_str(), url).await.unwrap();

    Json(UrlPostResponse {
        status: ResponseStatus::Accepted,
        short_url: key,
    })
}

#[get("/url/<key>")]
pub async fn get_url<'a>(key: &'a str, state: &State<ApplicationState>) -> Json<UrlGetResponse> {
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
