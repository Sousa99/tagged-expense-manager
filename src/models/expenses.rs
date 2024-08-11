use crate::utils::timestamp_serializer;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::expenses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Expense {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub title: String,
    pub description: Option<String>,
    pub timestamp: NaiveDateTime,
    pub value_decimal: i32
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::expenses)]
pub struct NewExpense {
    pub title: String,
    pub description: Option<String>,
    #[serde(with = "timestamp_serializer")]
    pub timestamp: NaiveDateTime,
    pub value_decimal: i32
}
