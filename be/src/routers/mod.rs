use crate::models::{ApplicationState, HealthResponse, HealthStatus, HelloResponse};

use rocket::serde::json::Json;
use rocket::{get, State};

#[get("/hello/<msg>")]
pub fn get_hello<'a>(msg: &'a str, state: &State<ApplicationState>) -> Json<HelloResponse<'a>> {
    rocket::info!(
        "requests served: {} by {}",
        state
            .requests_served
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        state.service_id
    );

    Json(HelloResponse::new(msg, state.service_id))
}

#[get("/health")]
pub fn get_health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: HealthStatus::Ok,
    })
}
