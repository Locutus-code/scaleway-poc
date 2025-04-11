use crate::models::state::ApplicationState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub enum ResponseStatus {
    Accepted,
    Error,
    Done,
}

#[derive(Deserialize, Serialize)]
pub struct UrlPostRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct UrlPostResponse {
    pub status: ResponseStatus,
    pub short_url: String,
    pub long_url: String,
    pub request_id: Uuid,
    pub service_id: Uuid,
}

impl UrlPostResponse {
    pub fn new(
        state: &ApplicationState,
        status: ResponseStatus,
        short_url: String,
        long_url: String,
    ) -> UrlPostResponse {
        UrlPostResponse {
            status,
            short_url,
            long_url,
            request_id: Uuid::new_v4(),
            service_id: state.service_id,
        }
    }
}

#[derive(Serialize)]
pub struct UrlGetResponse {
    pub short_url: Option<String>,
    pub long_url: Option<String>,
    pub status: ResponseStatus,
}
