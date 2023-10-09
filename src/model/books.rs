use crate::db::schema::books;
use crate::model::user::User;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(User,foreign_key = user_id))]
#[diesel(table_name = books,check_for_backend(diesel::pg::Pg))]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub user_id: i32,
}

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = books,check_for_backend(diesel::pg::Pg))]
pub struct BookGet {
    pub id: i32,
    pub title: String,
    pub user_id: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = books,check_for_backend(diesel::pg::Pg))]
pub struct BookUpdate<'a> {
    pub title: Option<&'a str>,
}

#[derive(Insertable)]
#[diesel(table_name = books,check_for_backend(diesel::pg::Pg))]
pub struct UserNew<'a> {
    pub title: &'a str,
    pub user_id: i32,
}
