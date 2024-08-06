mod jamclub_csv;
mod error;
use dotenvy::dotenv;
use crate::jamclub_csv::csv_import;
use error::error::MyError;

#[async_std::main]
async fn main() -> Result<(), MyError> {
    dotenv().ok();

    return csv_import::run().await;
}
