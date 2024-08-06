
use super::csv_enums::*;
use jamclub_seaorm::model_controller::CollectiveManager;
use jamclub_seaorm::entities::collectives;
use super::ToSeaOrm;
use sea_orm::{DbErr, ActiveValue::{NotSet, Set}, DatabaseConnection};
use jamclub_seaorm::model_controller::MemberManager;
use futures::future::join_all;
use std::fmt;
use super::from_str_deserialize;


#[derive(Clone, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct CsvCollectives {
    #[serde(rename="RECORD_ID")]
    pub ref_airtable: Option<String>,
    #[serde(rename="REF Responsable")]
    pub organizer_airtable_id: String,
    #[serde(rename="intitulé court")]
    pub short_title: String,
    #[serde(rename="Intitulé")]
    pub long_title: String,
    #[serde(rename="Jam description")]
    pub presentation: String,
    #[serde(rename="Notes")]
    pub notes: Option<String>,
    #[serde(rename="DISCORD RoleID")]
    pub discord_role_id: Option<i64>,
    #[serde(rename="DISCORD ChannelID")]
    pub discord_channel_id: Option<i64>,
    #[serde(rename="discord_presentation_url")]
    pub discord_presentation_url: Option<String>,
    #[serde(rename="strCommunauté", deserialize_with="from_str_deserialize")]
    pub communities: Vec<CsvCommunity>,
    #[serde(rename="Archivé")]
    pub status: CsvArchivedStatus,
    #[serde(rename="Creation time")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename="Last modified time")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename="LISTE Référents lineup")]
    pub referent_airtable_ids: String,
    #[serde(rename="LISTE Inscrits")]
    pub subscriber_airtable_ids: String,

}

impl std::fmt::Display for CsvCollectives {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the output using the `write!` macro
        write!(f, "CsvCollectives ( ref_airtable: {:?} )", self.ref_airtable)
    }
}

impl ToSeaOrm<collectives::ActiveModel> for CsvCollectives {
    async fn to_sea_orm(&self, db: &DatabaseConnection) -> Result<collectives::ActiveModel, sea_orm::DbErr> {
        let collective = CollectiveManager::find_by_airtable_ref(self.ref_airtable.clone().unwrap(), db).await?;
        let mut id = NotSet;
        if let Some(c) = collective { id = Set(c.id) }; 
        Ok(collectives::ActiveModel {
            id: id,
            ref_airtable: Set(self.ref_airtable.clone()),
            organizer_id: 
                Set({
                        let result = MemberManager::find_by_airtable_ref(self.organizer_airtable_id.clone(), db).await?;
                        let member = result.ok_or_else(|| DbErr::RecordNotFound(format!("Member with ref_airtable {} not found", self.organizer_airtable_id)))?;
                        member.id
                })
            ,
            short_title: Set(self.short_title.clone()),
            long_title: Set(self.long_title.clone()),
            presentation: Set(self.presentation.clone()),
            notes: Set(self.notes.clone()),
            discord_role_id: Set(self.discord_role_id.clone()),
            discord_channel_id: Set(self.discord_channel_id.clone()),
            discord_presentation_url: Set(self.discord_presentation_url.clone()),
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