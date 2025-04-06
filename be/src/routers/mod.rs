use crate::models::{ApplicationState, HelloResponse};

use rocket::{get, State};
use rocket::serde::json::Json;


#[get("/hello/<msg>")]
pub fn get_hello(msg: &str) -> Json<HelloResponse> {
    Json(
	HelloResponse::new(msg)
    )
}

