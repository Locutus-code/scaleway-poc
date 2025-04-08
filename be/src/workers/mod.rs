use crate::models::ApplicationState;

use aws_config::{Region, SdkConfig};
use aws_sdk_sqs as sqs;

pub async fn create_aws_config(state: &ApplicationState) -> SdkConfig {
    aws_config::load_from_env()
        .await
        .to_builder()
        .endpoint_url(state.endpoint_url.clone())
        .region(Region::new("fr-par"))
        .build()
}

pub async fn get_sqs_client(state: &ApplicationState) -> sqs::Client {
    let config = create_aws_config(state).await;
    sqs::client::Client::new(&config)
}
pub async fn enqueue_hello_msg(state: &ApplicationState, message: &str) {
    let client = get_sqs_client(state).await;
    rocket::info!("Sending to {}", state.queue_url);
    let result = client
        .send_message()
        .message_body(message)
        .queue_url(state.queue_url.clone())
        .send()
        .await;
    match result {
        Ok(_) => rocket::info!("Message enqueued success"),
        Err(e) => rocket::error!("Message enqueue failed: {}", e),
    }
}
