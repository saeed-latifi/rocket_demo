#[macro_use]
extern crate rocket;

use proji::provider::user::UserProvider;
use rocket::{
    fs::{relative, FileServer},
    serde::json::{json, Value},
};

#[get("/")]
fn index() -> Value {
    let res = UserProvider::get_list(100, 0);
    match res {
        Ok(users) => json!(users),
        Err(_) => json!("internal server error"),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", FileServer::from(relative!("public")))
}
