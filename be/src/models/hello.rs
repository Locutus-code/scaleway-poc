use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct HelloResponse<'a> {
    pub msg: &'a str,
    pub request_id: Uuid,
    pub service_id: Uuid,
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
