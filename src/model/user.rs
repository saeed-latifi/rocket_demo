use crate::db::schema::users;
use diesel::prelude::*;

#[derive(AsChangeset)]
#[diesel(table_name = users,check_for_backend(diesel::pg::Pg))]
pub struct UserUpdate<'a> {
    pub username: Option<&'a str>,
    pub password: Option<&'a str>,
}

//
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = users,check_for_backend(diesel::pg::Pg))]
pub struct GetUser {
    pub id: i32,
    pub username: String,
}

//
#[derive(Insertable)]
#[diesel(table_name = users,check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
