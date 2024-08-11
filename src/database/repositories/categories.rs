use crate::database::entities::categories::{Category, NewCategory};
use crate::schema;

use diesel::prelude::*;

pub fn insert_new_category(
    new_category: NewCategory,
    conn: &mut SqliteConnection,
) -> QueryResult<Category> {
    // Save Category
    diesel::insert_into(schema::categories::table)
        .values(&new_category)
        .returning(Category::as_returning())
        .get_result(conn)
}
