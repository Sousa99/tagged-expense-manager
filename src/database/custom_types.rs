use diesel::sql_types::SqlType;

#[derive(SqlType)]
#[diesel(postgres_type(name = "value"))]
pub struct ValueSqlType;
