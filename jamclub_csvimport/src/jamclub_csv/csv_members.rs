use jamclub_seaorm::entities::members;
use jamclub_seaorm::model_controller::MemberManager;
use super::ToSeaOrm;
use sea_orm::ActiveValue::{Set, NotSet};
use sea_orm::{DbErr, DatabaseConnection};
use super::csv_enums::*;
use super::from_str_option_deserialize;
use super::from_str_space_separated_option_deserialize;
use super::from_str_option_roles_deserialize;
use super::from_str_bool_deserialize;
use super::from_str_deserialize;
use futures::future::join_all;
use std::fmt;

#[derive(Clone, Debug, serde::Deserialize, PartialEq,  Eq)]
pub struct CsvMembers {
    #[serde(rename="Prénom")]
    pub first_name: String,
    #[serde(rename="Nom")]
    pub last_name: String,
    #[serde(rename="Email")]
    pub email: String,
    #[serde(rename="Membre ZAP RECORD_ID")]
    pub ref_airtable: Option<String>,
    #[serde(rename="Pseudo ZAP")]
    pub nick_name: Option<String>,
    #[serde(rename="User ID Discord")]
    pub discord_user_id: Option<u64>,
    #[serde(rename="Pseudo Discord")]
    pub discord_nick_name: String,
    #[serde(rename="Téléphone")]
    pub phone_number: Option<String>,
    #[serde(rename="Code postal")]
    pub postal_code: Option<i32>,
    #[serde(rename="Adresse")]
    pub address: Option<String>,
    #[serde(rename="Présentation")]
    pub presentation: String,
    #[serde(rename="Remarques")]
    pub remarks: Option<String>,
    #[serde(default, rename="strInstruments", deserialize_with = "from_str_option_deserialize")]
    //#[serde(alias="strInstruments", deserialize_with = "from_str_option_deserialize")]
    pub played_instruments: Option<Vec<CsvPlayedInstrument>>,
    #[serde(default, rename="strStylesDeMusique", deserialize_with = "from_str_option_deserialize")]
    //#[serde(alias="strStylesDeMusique", deserialize_with = "from_str_option_deserialize")]
    pub music_genres: Option<Vec<CsvMusicGenre>>,
    #[serde(default, rename="Type(s) de rencontre", deserialize_with = "from_str_space_separated_option_deserialize")]
    //#[serde(alias="Type(s) de rencontre", deserialize_with = "from_str_option_deserialize")]
    pub encounter_types: Option<Vec<CsvEncounterType>>,
    #[serde(default, rename="Role MIP", deserialize_with = "from_str_option_roles_deserialize")]
    //#[serde(alias="Role MIP", deserialize_with = "from_str_option_deserialize")]
    pub roles: Option<Vec<CsvZapRole>>,
    #[serde(rename="Provenance")]
    pub provenance: CsvProvenance,
    #[serde(rename="Diffusion audio", deserialize_with = "from_str_bool_deserialize" )]
    pub audio_diffusion: Option<bool>,
    #[serde(rename="Diffusion Vidéo", deserialize_with = "from_str_bool_deserialize" )]
    pub video_diffusion: Option<bool>,
    #[serde(rename="Newsletter ?", deserialize_with = "from_str_bool_deserialize" )]
    pub newsletter: Option<bool>,
    //#[serde(alias="Statut", deserialize_with = "from_str_deserialize")]
    #[serde(rename="Date de naissance")]
    pub birth_date: chrono::NaiveDate,
    #[serde(rename="Role(s) Communauté(s)", deserialize_with="from_str_deserialize")]
    pub communities: Vec<CsvCommunity>,
    #[serde(rename="Statut")]
    pub status: CsvPublicationStatus,
    #[serde(rename="Created")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename="Dernière modification")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl std::fmt::Display for CsvMembers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the output using the `write!` macro
        write!(f, "CsvMembers ( ref_airtable: {:?} )", self.ref_airtable)
    }
}

impl ToSeaOrm<members::ActiveModel> for CsvMembers {
    async fn to_sea_orm(&self, db :&DatabaseConnection) -> Result<members::ActiveModel, DbErr> {
        let member = MemberManager::find_by_airtable_ref(self.ref_airtable.clone().unwrap(), db).await?;
        let mut id = NotSet;
        if let Some(c) = member { id = Set(c.id) };

        Ok(members::ActiveModel {
            id: id,
            first_name: Set(self.first_name.clone()),
            last_name: Set(self.last_name.clone()),
            birth_date: Set(self.birth_date),
            email: Set(self.email.clone()),
            ref_airtable: Set(self.ref_airtable.clone()),
            nick_name: Set(self.nick_name.clone()),
            discord_user_id: Set(self.discord_user_id.map(|id| id as i64)),
            discord_nick_name: Set(self.discord_nick_name.clone()),
            phone_number: Set(self.phone_number.clone()),
            postal_code: Set(self.postal_code),
            address: Set(self.address.clone()),
            presentation: Set(self.presentation.clone()),
            remarks: Set(self.remarks.clone()),
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
            encounter_types: {
                match &self.encounter_types {
                    Some(e) =>  {
                        let futures = e.into_iter().map(|enconter_type| enconter_type.to_sea_orm(db));
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

            roles: {
                match &self.roles {
                    Some(r) =>  {
                        let futures = r.into_iter().map(|role| role.to_sea_orm(db));
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
            provenance: Set(
                self.provenance.to_sea_orm(db).await
                .map_err(|_| DbErr::AttrNotSet("provenance could not be set".to_string()))?
            ),            
            audio_diffusion: Set(self.audio_diffusion), // Set default or derive from CSV if needed
            video_diffusion: Set(self.video_diffusion), // Set default or derive from CSV if needed
            newsletter: Set(self.newsletter), // Set default or derive from CSV if needed
            status: Set(
                self.status.to_sea_orm(db).await
                .map_err(|_| DbErr::AttrNotSet("status could not be set".to_string()))?
            ),
            created_at: Set(self.created_at),
            updated_at: Set(self.updated_at),
        })
    }
}
