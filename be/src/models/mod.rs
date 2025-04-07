use std::sync::atomic::AtomicU64;

// use rocket::State;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct HelloResponse<'a> {
    msg: &'a str,
    request_id: Uuid,
    service_id: Uuid,
}

impl HelloResponse<'_> {
    pub fn new(message: &str, service_id: Uuid) -> HelloResponse {
        HelloResponse {
            msg: message,
            request_id: Uuid::new_v4(),
            service_id,
        }
    }
}

#[derive(Serialize)]
pub enum HealthStatus {
    Ok,
    Error,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: HealthStatus,
}

pub struct ApplicationState {
    pub requests_served: AtomicU64,
    pub service_id: Uuid,
}

impl ApplicationState {
    pub fn new() -> ApplicationState {
        ApplicationState {
            requests_served: 0.into(),
            service_id: Uuid::new_v4(),
        }
    }
}
