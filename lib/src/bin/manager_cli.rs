use dotenvy::dotenv;
use tagged_expense_manager::database;

fn main() {
    // Load Environment Variables
    dotenv().expect("🚫 .env file could not found");

    // Initialize Logger
    env_logger::init();

    log::info!("Starting up main application 🚀");
    let _database_connection = database::utils::connection::establish_connection();
}
