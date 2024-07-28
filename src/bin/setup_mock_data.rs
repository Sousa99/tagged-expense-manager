use tagged_expense_manager::database;

use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;

fn main() {
    // Load Environment Variables
    dotenv().expect("🚫 .env file could not found");

    // Initialize Logger
    env_logger::init();

    log::info!("Starting up main application 🚀");
    let mut database_connection = database::connection::establish_connection();

    // Add Mock Expenses
    log::info!("Inserting mock 'expenses'");
    create_mock_expenses(&mut database_connection);
    log::info!("Completed inserting mock 'expenses'");
}

fn create_mock_expenses(conn: &mut SqliteConnection) {
    database::expenses::insert_new_expense(
        conn,
        "Test Expense 1",
        Some("Test Expense 1 Description"),
    )
    .unwrap();
    database::expenses::insert_new_expense(conn, "Test Expense 1", None).unwrap();
}
