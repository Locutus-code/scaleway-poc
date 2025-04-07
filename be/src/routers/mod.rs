use crate::models::{ApplicationState, HealthResponse, HealthStatus, HelloResponse};
use crate::workers::enqueue_hello_msg;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json;
use rocket::{State, get};

#[get("/hello/<msg>")]
pub async fn get_hello<'a>(
    msg: &'a str,
    state: &State<ApplicationState>,
) -> Json<HelloResponse<'a>> {
    rocket::info!(
        "requests served: {} by {}",
        state
            .requests_served
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        state.service_id
    );
    let msg = HelloResponse::new(msg, state.service_id);
    enqueue_hello_msg(state, serde_json::to_string(&msg).unwrap().as_str()).await;
    Json(msg)
}

#[get("/health")]
pub fn get_health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: HealthStatus::Ok,
    })
}
