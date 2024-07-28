use std::env;
use std::process;

use dotenvy::dotenv;
use diesel::prelude::*;

pub mod schema;
pub mod models;

fn main() {
    // Load Environment Variables
    dotenv().expect("🚫 .env file could not found");

    // Initialize Logger
    env_logger::init();

    log::info!("Starting up main application 🚀");

    // Establishing Connection with Database
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        log::error!("Environment variable 'DATABASE_URL' could not be loaded");
        process::exit(1);
    });

    log::debug!("Attempting to establish connection with: {}", database_url);
    let _database_connection = SqliteConnection::establish(&database_url).unwrap_or_else(|_| {
        log::error!("Connection could not be established with database");
        process::exit(1);
    });

    log::info!("Connection established with database");
}
