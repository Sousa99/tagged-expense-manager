use crate::database::entities::expenses::Expense;
use crate::database::repositories::expenses as repo_expenses;
use crate::database::utils as db_utils;

pub fn get_expenses() -> Option<Vec<Expense>> {
    let mut database_connection = db_utils::connection::establish_connection();

    match repo_expenses::get_expenses(&mut database_connection) {
        Ok(expenses) => Some(expenses),
        Err(err) => {
            log::error!("{}", err);
            None
        }
    }
}
