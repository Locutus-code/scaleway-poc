mod routers;
mod models;

use models::ApplicationState;
use rocket::routes;
use crate::routers::get_hello;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    // let state = ApplicationState::new();
    // println!("New run {}", state.service_id);
    
    let _rocket = rocket::build()
        .manage(ApplicationState::new())
	.mount("/api", routes![get_hello])
	.launch().await?;
    Ok(())
}
