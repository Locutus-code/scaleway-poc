#[macro_use]
extern crate rocket;

mod models;
mod routers;
mod workers;

use crate::models::state::ApplicationState;
use crate::routers::health::get_health;
use crate::routers::hello::get_hello;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let state = ApplicationState::new();
    let _rocket = rocket::build()
        .manage(state)
        .mount("/api", routes![get_hello])
        .mount("/", routes![get_health])
        .launch()
        .await?;
    Ok(())
}
