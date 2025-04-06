
use crate::models::{ApplicationState, HelloResponse};

use rocket::{get, State};
use rocket::serde::json::Json;


#[get("/hello/<msg>")]
pub fn get_hello<'a>(msg: &'a str, state: &State<ApplicationState>) -> Json<HelloResponse<'a>> {
    rocket::info!("requests served: {} by {}", state.requests_served.fetch_add(1, std::sync::atomic::Ordering::Relaxed), state.service_id);

    Json(
	HelloResponse::new(msg, state.service_id)
    )
}

