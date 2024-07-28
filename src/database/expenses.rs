use crate::models::expenses::{Expense, NewExpense};
use crate::schema;

use diesel::prelude::*;

pub fn insert_new_expense(
    conn: &mut SqliteConnection,
    title: &str,
    description: Option<&str>,
) -> QueryResult<Expense> {
    // Create Expense
    let new_expense = NewExpense { title, description };

    // Save Expense
    diesel::insert_into(schema::expenses::table)
        .values(&new_expense)
        .returning(Expense::as_returning())
        .get_result(conn)
}
