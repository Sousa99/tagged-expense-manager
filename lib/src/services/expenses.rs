use crate::database::entities::expenses::{Expense, NewExpense};
use crate::database::repositories::expenses as repo_expenses;
use crate::database::utils as db_utils;
use crate::database::utils::connection::DBConnection;
use crate::error::get_failure::GetFailure;
use crate::error::Error;
use crate::error::Result;

pub fn get_expenses(database_connection: Option<&mut DBConnection>) -> Result<Vec<Expense>> {
    db_utils::connection::with_connection(database_connection, |conn| {
        repo_expenses::get_expenses(conn).map_err(|op| {
            Error::from(GetFailure {
                entity: String::from("EXPENSES"),
                query_key: String::from("GET_ALL"),
                database_error: op,
            })
        })
    })
}

pub fn save_expenses(
    database_connection: Option<&mut DBConnection>,
    _expenses: Vec<NewExpense>,
) -> Result<Vec<Expense>> {
    db_utils::connection::with_connection(database_connection, |_conn| {
        todo!("Add main functionality!")
    })
}
