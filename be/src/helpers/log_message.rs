use crate::models::state::ApplicationState;

pub fn request_counter(state: &ApplicationState) {
    rocket::info!(
        "requests served: {} by {}",
        state
            .requests_served
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        state.service_id
    );
}
