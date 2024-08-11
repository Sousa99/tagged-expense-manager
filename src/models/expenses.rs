use diesel::prelude::*;
use serde::Deserialize;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::expenses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Expense {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::expenses)]
pub struct NewExpense {
    pub title: String,
    pub description: Option<String>,
}
