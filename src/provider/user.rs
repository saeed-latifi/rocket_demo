use crate::db::schema::users::*;
use crate::model::user::UserUpdate;
use crate::{
    db::connection::establish_connection,
    model::user::{UserGet, UserNew},
};
use diesel::result::Error;
use diesel::*;

pub fn get_users_list(take: i64, skip: i64) -> Result<Vec<UserGet>, Error> {
    let connection = &mut establish_connection();

    table
        .limit(take)
        .order(id)
        .offset(skip)
        .select(UserGet::as_returning())
        .load(connection)
}

pub fn get_user(user_id: i32) -> Result<UserGet, Error> {
    let connection = &mut establish_connection();

    table
        .find(user_id)
        .select(UserGet::as_select())
        .first(connection)
}

pub fn create_user(new_user: &UserNew) -> Result<UserGet, Error> {
    let connection = &mut establish_connection();

    diesel::insert_into(table)
        .values(new_user)
        .returning(UserGet::as_returning())
        .get_result(connection)
}

pub fn update_user(user_id: i32, update_data: &UserUpdate) -> Result<UserGet, Error> {
    let connection = &mut establish_connection();

    diesel::update(table)
        .filter(id.eq(user_id))
        .set(update_data)
        .returning(UserGet::as_select())
        .get_result(connection)
}

pub fn delete_user(user_id: i32) -> Result<UserGet, Error> {
    let connection = &mut establish_connection();

    diesel::delete(table)
        .filter(id.eq(user_id))
        .returning(UserGet::as_select())
        .get_result(connection)
}
