use dotenvy::dotenv;
use tagged_expense_manager::database::utils as db_utils;

fn main() {
    // Load Environment Variables
    dotenv().expect("🚫 .env file could not found");
    // Initialize Logger
    env_logger::init();

    log::info!("Starting up terminal command line interface 🚀");

    log::debug!("Attempting to establish connection with database");
    let mut _database_connection = db_utils::connection::establish_connection();
    log::debug!("Connection with database established");
}
