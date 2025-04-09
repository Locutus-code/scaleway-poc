use std::env;
use std::sync::atomic::AtomicU64;
use uuid::Uuid;

pub struct ApplicationState {
    pub requests_served: AtomicU64,
    pub service_id: Uuid,
    pub queue_url: String,
    pub endpoint_url: String,
}

impl ApplicationState {
    pub fn new() -> ApplicationState {
        let queue_url = env::var("PRODUCER_QUEUE_URL").expect("queue url must be set");
        let endpoint_url = env::var("ENDPOINT_URL").expect("endpoint url must be set");
        ApplicationState {
            requests_served: 0.into(),
            service_id: Uuid::new_v4(),
            queue_url,
            endpoint_url,
        }
    }
}
