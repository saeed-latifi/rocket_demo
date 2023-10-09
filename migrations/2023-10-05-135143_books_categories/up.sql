CREATE TABLE books_categories (
    book_id INTEGER REFERENCES books(id),
    categories_id INTEGER REFERENCES categories(id),
    PRIMARY KEY(book_id, categories_id)
);