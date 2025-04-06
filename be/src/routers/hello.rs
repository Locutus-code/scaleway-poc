#[get("/hello/<name>")]
pub fn get_hello(name: &str) -> String {
    format!("Hi {}", name)
}
