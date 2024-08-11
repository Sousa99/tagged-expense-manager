use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Category {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::categories)]
pub struct NewCategory {
    pub name: String,
    pub description: Option<String>,
}
