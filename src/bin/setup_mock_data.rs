use tagged_expense_manager::database;
use tagged_expense_manager::models::expenses::{Expense, NewExpense};

use std::fs::File;
use std::path::{Path, PathBuf};
use std::process;

use clap::Parser;
use diesel::{QueryResult, SqliteConnection};
use dotenvy::dotenv;
use serde::de::DeserializeOwned;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to the folder of mock files to use
    #[arg(short, long)]
    mock_path: std::path::PathBuf,
}

fn main() {
    // Load Environment Variables
    dotenv().expect("🚫 .env file could not found");
    // Initialize Logger
    env_logger::init();
    // Load Arguments
    let args = Cli::parse();

    log::info!("Starting up main application 🚀");
    let mut database_connection = database::connection::establish_connection();

    import_from_csv::<NewExpense, Expense>(
        &args.mock_path,
        String::from("expenses"),
        database::expenses::insert_new_expense,
        &mut database_connection,
    );
}

fn import_from_csv<T, U>(
    folder: &Path,
    entity_identifier: String,
    insert_function: fn(T, &mut SqliteConnection) -> QueryResult<U>,
    conn: &mut SqliteConnection,
) where
    T: DeserializeOwned,
{
    let filename: String = format!("{}.csv", entity_identifier);
    let filepath: PathBuf = folder.join(filename);

    // Parse Records from CSV
    let records: Vec<T> = parse_records(&filepath, &entity_identifier);

    // Ingest Records Read
    log::debug!("Inserting mock '{}'", entity_identifier);

    records
        .into_iter()
        .map(|record| insert_function(record, conn))
        .enumerate()
        .filter_map(|(index, result)| match result {
            Ok(_) => None,
            Err(err) => Some((index, err)),
        })
        .for_each(|(index, err)| {
            log::error!("Record '#{}' could not be inserted: {}", index + 1, err)
        });

    log::info!("Completed inserting mock '{}'", entity_identifier);
}

fn parse_records<T>(filepath: &PathBuf, entity_identifier: &String) -> Vec<T>
where
    T: DeserializeOwned,
{
    log::debug!("Reading mock '{}'", entity_identifier);

    let records: Vec<T> = read_records(filepath).unwrap_or_else(|err| {
        log::error!("Error reading mock 'expenses'");
        log::error!("{}", err);
        process::exit(1);
    });

    log::info!("Completed reading mock '{}", entity_identifier);
    records
}

fn read_records<T>(filepath: &PathBuf) -> Result<Vec<T>, String>
where
    T: DeserializeOwned,
{
    let file = File::open(filepath).map_err(|err| err.to_string())?;
    let mut reader = csv::Reader::from_reader(file);

    let records: csv::Result<Vec<T>> = reader.deserialize().collect();
    records.map_err(|err| err.to_string())
}
