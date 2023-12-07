use diesel::PgConnection;
use rocket_sync_db_pools::database;

#[database("postgresConnection")]
pub struct DBConnection(PgConnection);
