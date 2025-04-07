#[macro_use]
extern crate rocket;

mod routers;
mod models;


use models::ApplicationState;


use crate::routers::{get_hello, get_health};


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let state = ApplicationState::new();
    let _rocket = rocket::build()
        .manage(state)
	.mount("/api", routes![get_hello])
	.mount("/", routes![get_health])
	.launch().await?;
    Ok(())
}
