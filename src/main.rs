use dotenvy::dotenv;

fn main() {
    // Load Environment Variables
    dotenv().expect(".env file could not found");

    // Initialize Logger
    env_logger::init();

    log::info!("Starting up main application 🚀");
}
