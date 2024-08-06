use super::csv_enums::*;
use jamclub_seaorm::entities::events;
use super::ToSeaOrm;
use sea_orm::ActiveValue::{Set, NotSet};
use sea_orm::{DbErr, DatabaseConnection};
use futures::future::join_all;
use jamclub_seaorm::model_controller::{MemberManager, LocationManager, EventManager};
use std::fmt;
use super::from_str_deserialize;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, Eq)]
pub struct CsvEvents {
    #[serde(rename="RECORD_ID")]
    pub ref_airtable: Option<String>,
    #[serde(rename="LISTE_Organisateurs")]
    pub organizer_airtable_id: String,
    #[serde(rename="Intitulé")]
    pub title: String,
    #[serde(rename="Description")]
    pub presentation: String,
    #[serde(skip_deserializing)]
    //#[serde(alias="", deserialize_with="from_str_option_deserialize")]
    pub music_genres: Option<Vec<CsvMusicGenre>>,
    #[serde(rename="Statut")]
    pub status: CsvEventStatus,
    #[serde(rename="Type", deserialize_with="from_str_deserialize")]
    pub event_types: Vec<CsvEventType>,
    #[serde(rename="REF_Lieu")]
    pub location_airtable_id: Option<String>,
    #[serde(rename="LISTE Lineups")]
    pub lineup_airtable_ids: String,
    #[serde(rename="Lien Mobilizon")]
    pub mobilizon_url: Option<String>,
    #[serde(skip_deserializing)]
    pub facebook_url: Option<String>,
    #[serde(rename="Date")]
    pub start_date: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename="Created")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename="Modified")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl std::fmt::Display for CsvEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the output using the `write!` macro
        write!(f, "CsvEvents ( ref_airtable: {:?} )", self.ref_airtable)
    }
}


impl ToSeaOrm<events::ActiveModel> for CsvEvents {
    async fn to_sea_orm(&self, db :&DatabaseConnection) -> Result<events::ActiveModel, DbErr> {
        let event = EventManager::find_by_airtable_ref(self.ref_airtable.clone().unwrap(), db).await?;
        let mut id = NotSet;
        if let Some(c) = event { id = Set(c.id) }; 

        Ok(events::ActiveModel {
            id: id,
            ref_airtable: Set(self.ref_airtable.clone()),
            organizer_id:
                Set({
                    let result = MemberManager::find_by_airtable_ref(self.organizer_airtable_id.clone(), db).await?;
                    let member = result.ok_or_else(|| DbErr::RecordNotFound(format!("Member with ref_airtable {} not found", self.organizer_airtable_id)))?;
                    member.id
            })
            ,
            title: Set(self.title.clone()),
            presentation: Set(self.presentation.clone()),
            music_genres: {
                match &self.music_genres {
                    Some(p) =>  {
                        let futures = p.into_iter().map(|music_genre| music_genre.to_sea_orm(db));
                        let results = join_all(futures).await;
                        let mut collected_results = Vec::with_capacity(results.len());
                        for result in results {
                            match result {
                                Ok(music_genre) => collected_results.push(music_genre),
                                Err(e) => return Err(e), // Propagate the error if any
                            }
                        }
                        Set(Some(collected_results))
                    },
                    _ => Set(None),
                }
            },
            status: Set(
                self.status.to_sea_orm(db).await
                .map_err(|_| DbErr::AttrNotSet("status could not be set".to_string()))?
            ),
            location_id: 
                {   
                    match self.location_airtable_id.as_deref() {
                        Some("rec4MK4EixPgWZRaS") => Set(None),
                        Some(loc_ref) => {
                                let result = LocationManager::find_by_airtable_ref(loc_ref.to_string(), db).await?;
                                let location = result.ok_or_else(|| DbErr::RecordNotFound(format!("Location with ref_airtable {} not found", &loc_ref)))?;
                                Set(Some(location.id))
                            },
                        None => Set(None),
                    }
                }
            ,
            event_types: Set(
                {
                    let event_types_futures: Vec<_> = self.event_types.iter()
                    .map(|event_type| event_type.to_sea_orm(db))
                    .collect();
        
                    // Await all the futures
                    let results = join_all(event_types_futures).await;
                    // Collect the results, propagating any errors
                    let mut collected_results = Vec::with_capacity(results.len());
                    for result in results {
                        match result {
                            Ok(event_type) => collected_results.push(event_type),
                            Err(e) => return Err(e), // Propagate the error if any
                        }
                    }
                    collected_results
                }
            ),
            mobilizon_url: Set(self.mobilizon_url.clone()),
            facebook_url: NotSet,
            start_date: Set(self.start_date),
            created_at: Set(self.created_at),
            updated_at: Set(self.updated_at),
        })
    }
}