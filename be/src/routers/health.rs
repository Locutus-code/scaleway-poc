use rocket::get;
use rocket::serde::json::Json;

use crate::models::health::{HealthResponse, HealthStatus};

#[get("/health")]
pub fn get_health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: HealthStatus::Ok,
    })
}
