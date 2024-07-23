use std::env;
use diesel::{PgConnection, Connection};

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in path");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error Connecting to {}", database_url))

}