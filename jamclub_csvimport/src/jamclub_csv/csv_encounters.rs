use super::csv_enums::*;
use jamclub_seaorm::entities::encounters;
use super::ToSeaOrm;
use sea_orm::{DbErr, ActiveValue::{NotSet, Set}, DatabaseConnection};
use futures::future::join_all;
use jamclub_seaorm::model_controller::{MemberManager, LocationManager, CollectiveManager, EncounterManager};
use super::from_str_deserialize;
use super::from_str_option_deserialize;
use super::from_str_bool_deserialize;
use std::fmt;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, Eq)]
pub struct CsvEncounters {
    #[serde(rename="REF Organisateur")]
    pub organizer_airtable_id: String,
    #[serde(rename="RECORD_ID")]
    pub ref_airtable: Option<String>,
    #[serde(rename="Intitulé")]
    pub short_title: String,
    #[serde(rename="Intitulé long")]
    pub long_title: String,
    #[serde(rename="Description")]
    pub presentation: String,
    #[serde(rename="Notes")]
    pub notes: Option<String>,
    #[serde(rename="Instrument(s) recherché(s)", deserialize_with="from_str_option_deserialize")]
    pub played_instruments: Option<Vec<CsvPlayedInstrument>>,
    #[serde(rename="Styles de musique", deserialize_with="from_str_option_deserialize")]
    pub music_genres: Option<Vec<CsvMusicGenre>>,
    #[serde(rename="Type de rencontre")]
    pub encounter_type: CsvEncounterType,
    #[serde(rename="DISCORD Message URL")]
    pub discord_url_message: Option<String>,
    #[serde(rename="DISCORD MessageID")]
    pub discord_message_id: Option<i64>,
    #[serde(rename="DISCORD ThreadID")]
    pub discord_thread_id: Option<i64>,
    #[serde(rename="DISCORD ChannelID")]
    pub discord_channel_id: Option<i64>,
    #[serde(rename="strCommunautés", deserialize_with="from_str_deserialize")]
    pub communities: Vec<CsvCommunity>,
    #[serde(rename="REF Lieu")]
    pub location_airtable_id: Option<String>,
    #[serde(rename="REF Collectif")]
    pub collective_airtable_id: Option<String>,
    #[serde(rename="Statut")]
    pub status: CsvEncounterStatus,
    #[serde(rename="Date de Rencontre")]
    pub start_date: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename="Fin Rencontre")]
    pub end_date: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename="Rappel 14 jours avant fait ?", deserialize_with="from_str_bool_deserialize")]
    pub reminder_done_14days_bef: Option<bool>,
    #[serde(rename="Rappel 7 jours avant fait ?", deserialize_with="from_str_bool_deserialize")]
    pub reminder_done_7days_bef: Option<bool>,
    #[serde(rename="Rappel 2 heures avant fait ?", deserialize_with="from_str_bool_deserialize")]
    pub reminder_done_2hours_bef: Option<bool>,
    #[serde(rename="Inscription ouverte jusqu'au dernier moment", deserialize_with="from_str_bool_deserialize")]
    pub registration_until_last_time: Option<bool>,
    #[serde(rename="Created time")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename="Last modified time")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename="Liste des absents")]
    pub absent_airtable_ids: String,
    #[serde(rename="LISTE Participants")]
    pub participant_airtable_ids: String,
    #[serde(rename="Participants Non dispo")]
    pub participant_not_available_airtable_ids: String,
    #[serde(rename="Participants incertain (ou disponible si beoin)")]
    pub participant_if_needed_airtable_ids: String,

}

impl std::fmt::Display for CsvEncounters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the output using the `write!` macro
        write!(f, "CsvEncounters ( ref_airtable: {:?} )", self.ref_airtable)
    }
}


impl ToSeaOrm<encounters::ActiveModel> for CsvEncounters {
    async fn to_sea_orm(&self, db :&DatabaseConnection) -> Result<encounters::ActiveModel, sea_orm::DbErr> {
        let encounter = EncounterManager::find_by_airtable_ref(self.ref_airtable.clone().unwrap(), db).await?;
        let mut id = NotSet;
        if let Some(c) = encounter { id = Set(c.id) }; 

        Ok(encounters::ActiveModel {
            id: id,
            organizer_id: 
                Set({
                    let result = MemberManager::find_by_airtable_ref(self.organizer_airtable_id.clone(), db).await?;
                    let member = result.ok_or_else(|| DbErr::RecordNotFound(format!("Member with ref_airtable {} not found", self.organizer_airtable_id)))?;
                    member.id
                })
            ,
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
            ref_airtable: Set(self.ref_airtable.clone()),
            short_title: Set(self.short_title.clone()),
            long_title: Set(self.long_title.clone()),
            presentation: Set(self.presentation.clone()),
            notes: Set(self.notes.clone()),
            played_instruments: {
                match &self.played_instruments {
                    Some(p) =>  {
                        let futures = p.into_iter().map(|instrument| instrument.to_sea_orm(db));
                        let results = join_all(futures).await;
                        
                                           // Collect the results, propagating any errors
                        let mut collected_results = Vec::with_capacity(results.len());
                        for result in results {
                            match result {
                                Ok(played_instrument) => collected_results.push(played_instrument),
                                Err(e) => return Err(e), // Propagate the error if any
                            }
                        }
                        Set(Some(collected_results))
                    },
                    _ => Set(None),
                }
            },
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
            encounter_type: Set(
                self.encounter_type.to_sea_orm(db).await
                .map_err(|_| DbErr::AttrNotSet("encounter type could not be set".to_string()))?
            ),
            discord_url_message: Set(self.discord_url_message.clone()),
            discord_message_id: Set(self.discord_message_id.clone()),
            discord_thread_id: Set(self.discord_thread_id.clone()),
            discord_channel_id: Set(self.discord_channel_id.clone()),
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
            start_date: Set(self.start_date),
            end_date: Set(self.end_date),
            reminder_done_14days_bef: Set(self.reminder_done_14days_bef.clone()),
            reminder_done_7days_bef: Set(self.reminder_done_7days_bef.clone()),
            reminder_done_2hours_bef: Set(self.reminder_done_2hours_bef.clone()),
            registration_until_last_time: Set(self.registration_until_last_time.clone()),
            communities: Set(
                {
                    let communities_futures: Vec<_> = self.communities.iter()
                    .map(|community| community.to_sea_orm(db))
                    .collect();
        
                    // Await all the futures
                    let results = join_all(communities_futures).await;
                    // Collect the results, propagating any errors
                    let mut collected_results = Vec::with_capacity(results.len());
                    for result in results {
                        match result {
                            Ok(volunteer_type) => collected_results.push(volunteer_type),
                            Err(e) => return Err(e), // Propagate the error if any
                        }
                    }
                    collected_results
                }
            ),
            status: Set(
                self.status.to_sea_orm(db).await
                .map_err(|_| DbErr::AttrNotSet("status could not be set".to_string()))?
            ),
            created_at: Set(self.created_at),
            updated_at: Set(self.updated_at),
        })
    }
}