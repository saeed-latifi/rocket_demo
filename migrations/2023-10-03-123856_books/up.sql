CREATE TABLE books(
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users(id)
)