// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        user_id -> Int4,
    }
}

diesel::table! {
    books_categories (book_id, categories_id) {
        book_id -> Int4,
        categories_id -> Int4,
    }
}

diesel::table! {
    categories (id) {
        id -> Int4,
        title -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        is_active -> Bool,
    }
}

diesel::joinable!(books -> users (user_id));
diesel::joinable!(books_categories -> books (book_id));
diesel::joinable!(books_categories -> categories (categories_id));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    books_categories,
    categories,
    users,
);
