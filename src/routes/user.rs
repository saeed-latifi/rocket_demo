use crate::{db::connection::DBConnection, provider::user::UserProvider};
use rocket::get;
use rocket::serde::json::{json, Value};

#[get("/")]
pub async fn get_list(db: DBConnection) -> Value {
    let res = db
        .run(move |connection| UserProvider::get_list(connection, 100, 0))
        .await;

    match res {
        Ok(users) => json!(users),
        Err(_) => json!("internal server error"),
    }
}
