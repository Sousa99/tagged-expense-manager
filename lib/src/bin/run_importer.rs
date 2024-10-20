use std::process::exit;

use tagged_expense_manager::database::utils as db_utils;
use tagged_expense_manager::services::expenses as service_expenses;
use tagged_expense_manager::importers::Importer;

use clap::Parser;
use dotenvy::dotenv;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The importer to use
    #[arg(short, long)]
    importer: Importer,
}

fn main() {
    // Load Environment Variables
    dotenv().expect("🚫 .env file could not found");
    // Initialize Logger
    env_logger::init();
    // Load Arguments
    let args = Cli::parse();

    log::info!("Starting up main application 🚀");
    let mut database_connection = db_utils::connection::establish_connection();

    let mut importer = args.importer.get_importer();
    log::info!("Selected importer '{}'", importer.get_name());

    log::debug!("Starting importer configuration");
    importer.configure()
        .unwrap_or_else(|_| {
            log::error!("Importer was not configured accordingly");
            exit(1);
        });

    log::debug!("Starting importer");
    let new_expenses = importer.import_expenses();
    log::info!("Importer successfully imported '#{}' expenses", new_expenses.len());

    log::info!("Saving expenses to database");
    let _expenses = service_expenses::save_expenses(Some(&mut database_connection), new_expenses);
    
}