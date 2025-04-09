use aws_sdk_sqs as sqs;

use crate::models::state::ApplicationState;
use crate::workers::aws::create_aws_config;

pub async fn get_sqs_client(state: &ApplicationState) -> sqs::Client {
    let config = create_aws_config(state).await;
    sqs::client::Client::new(&config)
}
pub async fn enqueue_msg(state: &ApplicationState, message: &str) {
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
