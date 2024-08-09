use tagged_expense_manager::{database, models::expenses::NewExpense};

use std::fs::File;
use std::process;

use dotenvy::dotenv;

fn main() {
    // Load Environment Variables
    dotenv().expect("🚫 .env file could not found");

    // Initialize Logger
    env_logger::init();

    log::info!("Starting up main application 🚀");
    let mut database_connection = database::connection::establish_connection();

    // Reading Mock Expenses
    log::debug!("Reading mock 'expenses'");
    let records = read_records("./mock-data/basic-functionality/expenses.csv".to_string())
        .unwrap_or_else(|err| {
            log::error!("Error reading mock 'expenses'");
            log::error!("{}", err);
            process::exit(1);
        });
    log::info!("Completed reading mock 'expenses");

    // Add Mock Expenses
    log::debug!("Inserting mock 'expenses'");
    records.into_iter()
        .map(|record| (record.title.clone(), database::expenses::insert_new_expense(&mut database_connection, record)))
        .filter_map(|(title, result)| {
            match result {
                Ok(_) => None,
                Err(err) => Some((title, err))
            }
        })
        .for_each(|(title, err)| log::error!("Record '{}' could not be inserted: {}", title, err));
    log::info!("Completed inserting mock 'expenses'");
}

fn read_records(filename: String) -> Result<Vec<NewExpense>, String> {
    let file = File::open(filename).map_err(|err| err.to_string())?;
    let mut reader = csv::Reader::from_reader(file);

    let records: csv::Result<Vec<NewExpense>> = reader.deserialize().into_iter().collect();
    records.map_err(|err| err.to_string())
}