use std::fmt::Display;

use argon2::Argon2;
use axum::{
    body::{Body, to_bytes},
    extract::Request,
    response::Response,
};
use http::StatusCode;

use common::SqsPayload;

fn handle_error<Err: Display>(err: Err, status: Option<StatusCode>) -> Response<Body> {
    let status = status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    Response::builder()
        .status(status)
        .body(Body::from(format!("{}", err)))
        .unwrap()
}

pub async fn handle(req: Request<Body>) -> Response<Body> {
    let body = to_bytes(req.into_body(), usize::MAX).await.unwrap();
    let result = serde_json::from_slice::<SqsPayload>(&body);

    let result_data = match result {
        Ok(data) => data,
        Err(error) => return handle_error(error, None),
    };

    let salt = "urmomaksjdklajsdlkajsdkljasldja".as_bytes();
    let mut output_key_material = [0u8; 32];
    let _ = Argon2::default().hash_password_into(
        result_data.msg.as_bytes(),
        salt,
        &mut output_key_material,
    );
    println!("{:?}", output_key_material);

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(Body::empty())
        .unwrap()
}
