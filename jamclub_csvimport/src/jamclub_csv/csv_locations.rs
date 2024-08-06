use sea_orm::prelude::Decimal;
use super::csv_enums::*;
use jamclub_seaorm::entities::locations;
use jamclub_seaorm::model_controller::LocationManager;
use super::ToSeaOrm;
use sea_orm::ActiveValue::{Set, NotSet};
use sea_orm::{DbErr,DatabaseConnection};
use futures::future::join_all;
use super::from_str_deserialize;
use std::fmt;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, Eq)]
pub struct CsvLocations {
    #[serde(alias="RECORD_ID")]
    pub ref_airtable: Option<String>,
    #[serde(alias="Intitulé court")]
    pub short_title: String,
    #[serde(alias="Intitulé long")]
    pub long_title: String,
    #[serde(alias="Adresse")]
    pub address: String,
    #[serde(alias="Code postal")]
    pub postal_code: i32,
    #[serde(alias="Google Maps URL")]
    pub google_maps_url: String,
    #[serde(alias="Latitude")]
    pub latitude: Option<Decimal>,
    #[serde(alias="Longitude")]
    pub longitude: Option<Decimal>,
    #[serde(alias="Notes")]
    pub notes: Option<String>,
    #[serde(alias="Téléphone")]
    pub phone_number: Option<String>,
    #[serde(alias="Email")]
    pub email: Option<String>,
    #[serde(alias="Archivé")]
    pub status: CsvArchivedStatus,
    #[serde(alias="Type de Lieu", deserialize_with="from_str_deserialize")]
    pub location_types: Vec<CsvLocationType>,
    #[serde(alias="Created")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}

impl std::fmt::Display for CsvLocations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the output using the `write!` macro
        write!(f, "CsvLocations ( ref_airtable: {:?} )", self.ref_airtable)
    }
}

impl ToSeaOrm<locations::ActiveModel> for CsvLocations {
    async fn to_sea_orm(&self, db :&DatabaseConnection) -> Result<locations::ActiveModel, DbErr> {
        let location = LocationManager::find_by_airtable_ref(self.ref_airtable.clone().unwrap(), db).await?;
        let mut id = NotSet;
        if let Some(c) = location { id = Set(c.id) }; 

        Ok(locations::ActiveModel {
            id: id,
            ref_airtable: Set(self.ref_airtable.clone()),
            short_title: Set(self.short_title.clone()),
            long_title: Set(self.long_title.clone()),
            address: Set(self.address.clone()),
            postal_code: Set(self.postal_code.clone()),
            google_maps_url: Set(self.google_maps_url.clone()),
            latitude: Set(self.latitude.clone()),
            longitude: Set(self.longitude.clone()),
            email: Set(self.email.clone()),
            phone_number: Set(self.phone_number.clone()),
            notes: Set(self.notes.clone()),
            status: Set(
                self.status.to_sea_orm(db).await
                .map_err(|_| DbErr::AttrNotSet("status could not be set".to_string()))?
            ),
            location_types:
                {
                    let location_types_futures: Vec<_> = self.location_types.iter()
                    .map(|location_type| location_type.to_sea_orm(db))
                    .collect();
        
                    // Await all the futures
                    let results = join_all(location_types_futures).await;
                    let mut collected_results = Vec::with_capacity(results.len());
                    for result in results {
                        match result {
                            Ok(music_genre) => collected_results.push(music_genre),
                            Err(e) => return Err(e), // Propagate the error if any
                        }
                    }
                    Set(collected_results)
                },
            created_at: Set(self.created_at),
        })
    }
}