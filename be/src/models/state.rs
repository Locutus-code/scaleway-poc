use std::env;
use std::sync::atomic::AtomicU64;
use uuid::Uuid;

pub struct ApplicationState {
    pub requests_served: AtomicU64,
    pub service_id: Uuid,
    pub queue_url: String,
    pub endpoint_url: String,
    pub redis_host: String,
    pub redis_port: u16,
    pub redis_username: String,
    pub redis_password: String,
}

impl ApplicationState {
    pub fn new() -> ApplicationState {
        let queue_url = env::var("PRODUCER_QUEUE_URL").expect("queue url must be set");
        let endpoint_url = env::var("ENDPOINT_URL").expect("endpoint url must be set");
        let redis_url = env::var("REDIS_URL").expect("REDIS url must be set");

        let parts: Vec<&str> = redis_url.split(":").collect();
        let redis_host = parts[0].to_string();
        let redis_port = parts[1].parse().unwrap();

        let redis_username = env::var("REDIS_USERNAME").expect("REDIS username must be set");
        let redis_password = env::var("REDIS_PASSWORD").expect("REDIS password must be set");
        ApplicationState {
            requests_served: 0.into(),
            service_id: Uuid::new_v4(),
            queue_url,
            endpoint_url,
            redis_host,
            redis_port,
            redis_username,
            redis_password,
        }
    }
}
