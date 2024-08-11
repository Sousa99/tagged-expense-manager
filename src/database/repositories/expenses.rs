use crate::database::entities::expenses::{Expense, NewExpense};
use crate::schema;

use diesel::prelude::*;

pub fn insert_new_expense(
    new_expense: NewExpense,
    conn: &mut SqliteConnection,
) -> QueryResult<Expense> {
    // Save Expense
    diesel::insert_into(schema::expenses::table)
        .values(&new_expense)
        .returning(Expense::as_returning())
        .get_result(conn)
}
