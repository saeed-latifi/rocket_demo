use diesel::{result::Error, *};

use crate::{
    db::{
        connection::establish_connection,
        schema::{books, users},
    },
    model::{books::BookGet, user::UserGet},
};

pub fn get_books_by_user_id(user_id: i32) -> Result<Vec<(BookGet, UserGet)>, Error> {
    let connection = &mut establish_connection();

    // simple way!
    // books::table
    //     .filter(books::user_id.eq(user_id))
    //     .select(BookGet::as_select())
    //     .load(connection)

    // join way
    books::table
        .inner_join(users::table)
        .filter(users::id.eq(user_id))
        .select((BookGet::as_select(), UserGet::as_select()))
        .load::<(BookGet, UserGet)>(connection)
}
