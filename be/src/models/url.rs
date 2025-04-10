use serde::Serialize;
// use uuid::Uuid;

#[derive(Serialize)]
pub enum ResponseStatus {
    Accepted,
    Error,
    InProgress,
    Done,
}

#[derive(Serialize)]
pub struct UrlPostResponse {
    pub status: ResponseStatus,
    pub short_url: String,
}

#[derive(Serialize)]
pub struct UrlGetResponse {
    pub short_url: Option<String>,
    pub long_url: Option<String>,
    pub status: ResponseStatus,
}
