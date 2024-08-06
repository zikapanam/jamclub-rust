
use sea_orm::DatabaseConnection;

pub trait ToSeaOrm<T> {
    async fn to_sea_orm(&self, db: &DatabaseConnection) -> Result<T, sea_orm::DbErr>;
}

pub mod csv_enums;
pub mod csv_members;
pub mod csv_locations;
pub mod csv_collectives;
pub mod csv_lineups;
pub mod csv_volunteers;
pub mod csv_encounters;
pub mod csv_events;
pub mod csv_import;
pub mod toseaorm_enums;
use serde::{Deserialize, Deserializer};
use serde::de::Error as DeError;
use std::str::FromStr;


pub fn from_str_bool_deserialize<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the string from the CSV field
    let s = String::deserialize(deserializer)?;
    
    match s.as_str() {
        "True" => Ok(Some(true)),
        "False" => Ok(Some(false)),
        _ => Ok(None),
    }
}


pub  fn from_str_deserialize<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: FromStr,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    // Deserialize the string from the CSV field
    let s = String::deserialize(deserializer)?;
    
    // Split the string by commas, trim whitespace, and parse each piece
    s.split(',')
        .map(|s| s.trim().parse::<T>().map_err(|e| DeError::custom(e)))
        .collect()
}

pub  fn from_str_option_deserialize<'de, T, D>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    T: FromStr,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            let parsed: Result<Vec<T>, _> = s.split(',')
                .map(|s| s.trim().parse::<T>().map_err(|e| DeError::custom(e)))
                .collect();
            parsed.map(Some)
        }
        None => Ok(None),
    }
}

pub  fn from_str_option_roles_deserialize<'de, T, D>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    T: FromStr,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            // Split the string by commas, parse each segment, and filter out errors
            let parsed: Result<Vec<T>, _> = s.split(',')
                .map(|s| s.trim()) // Trim whitespace
                .map(|s| T::from_str(s).map_err(|e| D::Error::custom(e.to_string())))
                .collect(); // Collect into a Result<Vec<T>, _>

            parsed.map(Some) // Wrap the result in `Some` if successful
                .map_err(DeError::custom) // Convert errors into `D::Error`
        }
        None => Ok(None),
    }
}

pub  fn from_str_space_separated_option_deserialize<'de, T, D>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    T: FromStr,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            let parsed: Result<Vec<T>, _> = s.split(' ')
                .map(|s| s.trim().parse::<T>().map_err(|e| DeError::custom(e)))
                .collect();
            parsed.map(Some)
        }
        None => Ok(None),
    }
}
