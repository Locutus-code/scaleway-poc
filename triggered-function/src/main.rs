use axum::{body::Body, http::Request};

mod handler;
use common::SqsPayload;

#[tokio::main]
async fn main() {
    let sample_msg = SqsPayload::mock();
    let json_payload = serde_json::to_string(&sample_msg).unwrap();
    let res = handler::handle(Request::new(Body::new(json_payload))).await;
    println!("{}", res.status());
}
