use crate::models::ApplicationState;

use aws_sdk_sqs as sqs;

pub async fn get_sqs_client() -> sqs::Client {
    let config = aws_config::load_from_env().await;
    sqs::client::Client::new(&config)
}
pub async fn enqueue_hello_msg(state: &ApplicationState, message: &str) {
    let client = get_sqs_client().await;
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
