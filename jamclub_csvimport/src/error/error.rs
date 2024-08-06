//! Main Crate Error
use sea_orm::DbErr;
use std::io;
#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] DbErr),

    #[error(transparent)]
    IoError(#[from] io::Error),

	#[error(transparent)]
    CsvError(#[from] csv::Error),  // Ensure CsvError variant is included

}