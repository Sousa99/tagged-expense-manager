use crate::database::entities::expenses::Expense;
use crate::database::repositories::expenses as repo_expenses;
use crate::database::utils as db_utils;
use crate::database::utils::connection::DBConnection;

pub fn get_expenses(database_connection: Option<&mut DBConnection>) -> Option<Vec<Expense>> {
    db_utils::connection::with_connection(database_connection, |conn| {
        match repo_expenses::get_expenses(conn) {
            Ok(expenses) => Some(expenses),
            Err(err) => {
                log::error!("{}", err);
                None
            }
        }
    })
}
