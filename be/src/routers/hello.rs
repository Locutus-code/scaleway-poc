use rocket::serde::json::Json;
use rocket::serde::json::serde_json;
use rocket::{State, get};

use crate::helpers::log_message::request_counter;
use crate::models::hello::HelloResponse;
use crate::models::state::ApplicationState;
use crate::workers::sqs::enqueue_msg;

use lib::{MessageType, SqsPayload};

#[get("/hello/<msg>")]
pub async fn get_hello<'a>(
    msg: &'a str,
    state: &State<ApplicationState>,
) -> Json<HelloResponse<'a>> {
    request_counter(state);
    let response = HelloResponse::new(msg, state.service_id);
    enqueue_msg(
        state,
        serde_json::to_string(&SqsPayload {
            msg_type: MessageType::Hello,
            msg: &msg,
            request_id: response.request_id,
            service_id: state.service_id,
        })
        .unwrap()
        .as_str(),
    )
    .await;
    Json(response)
}
