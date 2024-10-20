use diesel::result::Error as DieselError;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;

pub struct GetFailure {
    pub entity: String,
    pub query_key: String,
    pub database_error: DieselError,
}

impl Debug for GetFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetFailure")
            .field("entity", &self.entity)
            .field("query_key", &self.query_key)
            .field("database_error", &self.database_error)
            .finish()
    }
}

impl Display for GetFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "while retrieving '{}' with '{}' failed with '{}'",
            self.entity, self.query_key, self.database_error
        )
    }
}

impl Error for GetFailure {}
