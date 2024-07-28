use std::env;
use std::process;

use diesel::prelude::*;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        log::error!("Environment variable 'DATABASE_URL' could not be loaded");
        process::exit(1);
    });

    log::debug!("Attempting to establish connection with: {}", database_url);
    let database_connection = SqliteConnection::establish(&database_url).unwrap_or_else(|_| {
        log::error!("Connection could not be established with database");
        process::exit(1);
    });

    log::info!("Connection established with database");
    database_connection
}
