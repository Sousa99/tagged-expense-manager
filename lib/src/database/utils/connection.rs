use std::env;
use std::process;

use diesel::prelude::*;

pub use diesel::prelude::SqliteConnection as DBConnection;

pub fn establish_connection() -> DBConnection {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        log::error!("Environment variable 'DATABASE_URL' could not be loaded");
        process::exit(1);
    });

    log::debug!("Attempting to establish connection with: {}", database_url);
    let database_connection = DBConnection::establish(&database_url).unwrap_or_else(|_| {
        log::error!("Connection could not be established with database");
        process::exit(1);
    });

    log::info!("Connection established with database");
    database_connection
}

pub fn with_connection<F, R>(database_connection: Option<&mut DBConnection>, func: F) -> R
where
    F: FnOnce(&mut DBConnection) -> R,
{
    match database_connection {
        Some(conn) => func(conn),
        None => {
            let mut temp_conn = establish_connection();
            func(&mut temp_conn)
        }
    }
}
