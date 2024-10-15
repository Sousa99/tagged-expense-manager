use tagged_expense_manager::database::utils as db_utils;

pub use tagged_expense_manager::database::utils::connection::DBConnection;

pub fn establish_database_connection() -> DBConnection {
    log::debug!("Attempting to establish connection with database");
    let database_connection = db_utils::connection::establish_connection();
    log::debug!("Connection with database established");

    database_connection
}
