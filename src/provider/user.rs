use crate::db::schema::users::*;
use crate::model::user::UserUpdate;
use crate::{
    db::connection::establish_connection,
    model::user::{GetUser, NewUser},
};
use diesel::result::Error;
use diesel::*;

pub fn get_users_list(take: i64, skip: i64) -> Result<Vec<GetUser>, Error> {
    let connection = &mut establish_connection();

    table
        .limit(take)
        .order(id)
        .offset(skip)
        .select(GetUser::as_returning())
        .load(connection)
}

pub fn get_user(user_id: i32) -> Result<GetUser, Error> {
    let connection = &mut establish_connection();

    table
        .find(user_id)
        .select(GetUser::as_select())
        .first(connection)
}

pub fn create_user(new_user: &NewUser) -> Result<GetUser, Error> {
    let connection = &mut establish_connection();

    diesel::insert_into(table)
        .values(new_user)
        .returning(GetUser::as_returning())
        .get_result(connection)
}

pub fn update_user(user_id: i32, update_data: &UserUpdate) -> Result<GetUser, Error> {
    let connection = &mut establish_connection();

    diesel::update(table)
        .filter(id.eq(user_id))
        .set(update_data)
        .returning(GetUser::as_select())
        .get_result(connection)
}

pub fn delete_user(user_id: i32) -> Result<GetUser, Error> {
    let connection = &mut establish_connection();

    diesel::delete(table)
        .filter(id.eq(user_id))
        .returning(GetUser::as_select())
        .get_result(connection)
}
