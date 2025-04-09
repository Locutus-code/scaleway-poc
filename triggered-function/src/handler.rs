use std::fmt::Display;

use axum::{
    body::{to_bytes, Body}, extract::{Request}, response::Response
};
use http::StatusCode;
use lib::SqsPayload;


fn handle_error<Err: Display>(err: Err, status: Option<StatusCode>) -> Response<Body> {

    let status = status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    Response::builder()
        .status(status)
        .body(Body::from(format!("{}", err)))
        .unwrap()	
}

// async fn get_payload(extract::Json(payload): extract::Json<SqsPayload<'_>>) {}

pub async fn handle(req: Request<Body>) -> Response<Body> {

    let body = to_bytes(req.into_body(), usize::MAX).await.unwrap();
    let result = serde_json::from_slice::<SqsPayload>(&body);

    let result_data = match result {
	Ok(data) => data,
	Err(error) => return handle_error(error, None)
    };
    
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(Body::empty())
        .unwrap()
}
