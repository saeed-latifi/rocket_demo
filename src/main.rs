#[macro_use]
extern crate rocket;

use blog::{
    db::connection::DBConnection,
    routes::{
        greet::{greet, greet_with_name},
        user::get_list,
    },
};
use rocket::fs::{relative, FileServer};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("public")))
        .mount("/", routes![greet, greet_with_name])
        .mount("/user", routes![get_list,])
        .attach(DBConnection::fairing())
}
