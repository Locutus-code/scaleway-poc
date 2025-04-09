use crate::models::state::ApplicationState;
use aws_config::{Region, SdkConfig};

pub async fn create_aws_config(state: &ApplicationState) -> SdkConfig {
    aws_config::load_from_env()
        .await
        .to_builder()
        .endpoint_url(state.endpoint_url.clone())
        .region(Region::new("fr-par"))
        .build()
}
