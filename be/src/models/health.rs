use serde::Serialize;

#[allow(dead_code)]
#[derive(Serialize)]
pub enum HealthStatus {
    Ok,
    Error,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: HealthStatus,
}
