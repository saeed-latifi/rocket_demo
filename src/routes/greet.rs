use rocket::get;

#[get("/")]
pub fn greet() -> String {
    "Hello!".to_string()
}

#[get("/<name>")]
pub fn greet_with_name(name: &str) -> String {
    format!("Hello {} !", name)
}
