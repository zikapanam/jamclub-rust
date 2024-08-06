use super::csv_enums::*;
use jamclub_seaorm::entities::lineups;
use jamclub_seaorm::model_controller::{CollectiveManager, LineupManager};
use super::ToSeaOrm;
use sea_orm::ActiveValue::{Set, NotSet};
use sea_orm::{DbErr, DatabaseConnection};
use std::fmt;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, Eq)]
pub struct CsvLineups {
    #[serde(alias="RECORD_ID")]
    pub ref_airtable: Option<String>,
    #[serde(alias="REF Collectif")]
    pub collective_airtable_id: Option<String>,
    #[serde(alias="REF Référent")]
    pub organizer_airtable_id: String,
    #[serde(alias="Membres")]
    pub subscriber_airtable_ids: String,
    //#[serde(alias="Style(s) de musique", deserialize_with="from_str_deserialize")]
    #[serde(skip_deserializing)]
    pub music_genres: Vec<CsvMusicGenre>,
    #[serde(alias="intitulé court")]
    pub short_title: String,
    #[serde(alias="intitulé long")]
    pub long_title: String,
    #[serde(alias="Phrase d'accroche")]
    pub catch_phrase: String,
    #[serde(alias="Description")]
    pub presentation: String,
    #[serde(alias="Statut")]
    pub status: CsvPublicationStatus,
    #[serde(alias="Created")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(alias="Modified")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl std::fmt::Display for CsvLineups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the output using the `write!` macro
        write!(f, "CsvLineups ( ref_airtable: {:?} )", self.ref_airtable)
    }
}
impl ToSeaOrm<lineups::ActiveModel> for CsvLineups {
    async fn to_sea_orm(&self, db :&DatabaseConnection) -> Result<lineups::ActiveModel, DbErr> {
        let lineup = LineupManager::find_by_airtable_ref(self.ref_airtable.clone().unwrap(), db).await?;
        let mut id = NotSet;
        if let Some(c) = lineup { id = Set(c.id) }; 

        Ok(lineups::ActiveModel {
            id: id,
            ref_airtable: Set(self.ref_airtable.clone()),
            collective_id:
                Set({
                    match self.collective_airtable_id.as_deref() {
                        Some(coll_ref) => {
                            let result = CollectiveManager::find_by_airtable_ref(coll_ref.to_string(), db).await?;
                            let collective = result.ok_or_else(|| DbErr::RecordNotFound(format!("Collective with ref_airtable {} not found", &coll_ref)))?;
                            Some(collective.id)
                        },
                        None => None,
                    }   
                })
            ,
            short_title: Set(self.short_title.clone()),
            long_title: Set(self.long_title.clone()),
            presentation: Set(self.presentation.clone()),
            music_genres: NotSet,
                // {
                //     let music_genres_futures: Vec<_> = self.music_genres.iter()
                //     .map(|music_genre| music_genre.to_sea_orm(db))
                //     .collect();
        
                //     // Await all the futures
                //     let results = join_all(music_genres_futures).await;
                //     // Collect the results, propagating any errors
                //     let mut collected_results = Vec::with_capacity(results.len());
                //     for result in results {
                //         match result {
                //             Ok(music_genre) => collected_results.push(music_genre),
                //             Err(e) => return Err(e), // Propagate the error if any
                //         }
                //     }
                //     Set(collected_results)
                // },
            catch_phrase: Set(self.catch_phrase.clone()),
            status: Set(
                self.status.to_sea_orm(db).await
                .map_err(|_| DbErr::AttrNotSet("status could not be set".to_string()))?
            ),
            created_at: Set(self.created_at),
            updated_at: Set(self.updated_at),
        })
    }
}