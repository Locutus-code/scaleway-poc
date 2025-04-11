use serde::{Deserialize, Serialize};
// use uuid::Uuid;

#[derive(Serialize)]
pub enum ResponseStatus {
    Accepted,
    Error,
    InProgress,
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
}

#[derive(Serialize)]
pub struct UrlGetResponse {
    pub short_url: Option<String>,
    pub long_url: Option<String>,
    pub status: ResponseStatus,
}
