use diesel::result::Error as DieselError;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;

pub struct InsertFailure {
    entity: String,
    number_of_elements: usize,
    elements: Vec<usize>,
    database_error: DieselError,
}

impl Debug for InsertFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InsertFailure")
            .field("entity", &self.entity)
            .field("number_of_elements", &self.number_of_elements)
            .field("elements", &self.elements)
            .field("database_error", &self.database_error)
            .finish()
    }
}

impl Display for InsertFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "while inserting '#{}' of '{}' failed with '{}'",
            self.number_of_elements, self.entity, self.database_error
        )
    }
}

impl Error for InsertFailure {}
