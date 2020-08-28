mod models;
mod schema;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use std::env;

#[derive(Clone)]
pub struct DbContext {
    pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl DbContext {
    pub fn new_from_env() -> Self {
        let connection_string = env::var("DATABASE_URL").expect("DATABASE_URL environment variable was not set");
        DbContext::new(&connection_string)
    }

    pub fn new(connection_string: &str) -> Self {
        let pg_connection_manager = ConnectionManager::<PgConnection>::new(connection_string);
        let pool = r2d2::Pool::builder()
            .build(pg_connection_manager)
            .expect("Failed to create pool.");
        DbContext { pool }
    }
}
