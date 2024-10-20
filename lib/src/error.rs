use get_failure::GetFailure;
use insert_failure::InsertFailure;

use core::result::Result as CoreResult;
use std::fmt::Debug;
use thiserror::Error as ErrorDerive;

pub mod get_failure;
pub mod insert_failure;

#[derive(ErrorDerive, Debug)]
pub enum Error {
    #[error("Error getting entities from database: {0}")]
    GetFailure(#[from] GetFailure),
    #[error("Error inserting entities into database: {0}")]
    InsertFailure(#[from] InsertFailure),
}

pub type Result<T> = CoreResult<T, Error>;
