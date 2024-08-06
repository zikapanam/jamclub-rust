use super::csv_enums::*;
use jamclub_seaorm::entities::volunteers;
use super::ToSeaOrm;
use sea_orm::ActiveValue::{Set, NotSet};
use sea_orm::{DbErr, DatabaseConnection};
use futures::future::join_all;
use std::fmt;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, Eq)]
pub struct CsvVolunteers {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub nick_name: Option<String>,
    pub gallery_url: Option<String>,
    pub volunteer_types: Vec<CsvVolunteerType>,
    pub notes: Option<String>,
    pub status: CsvPublicationStatus,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl std::fmt::Display for CsvVolunteers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the output using the `write!` macro
        write!(f, "CsvVolunteers ( first_name: {}, last_name: {} )", self.first_name, self.last_name)
    }
}
impl ToSeaOrm<volunteers::ActiveModel> for CsvVolunteers {
    async fn to_sea_orm(&self, db :&DatabaseConnection) -> Result<volunteers::ActiveModel, DbErr> {
        // let member = MemberManager::find_by_airtable_ref(self.ref_airtable.clone().unwrap(), db).await?;
        // let mut id = NotSet;
        // if let Some(c) = member { id = Set(c.id) };


        Ok(volunteers::ActiveModel {
            id: NotSet,
            first_name: Set(self.first_name.clone()),
            last_name: Set(self.last_name.clone()),
            nick_name: Set(self.nick_name.clone()),
            gallery_url: Set(self.gallery_url.clone()),
            volunteer_types:
                {
                    let volunteer_types_futures: Vec<_> = self.volunteer_types.iter()
                    .map(|volunteer_type| volunteer_type.to_sea_orm(db))
                    .collect();
        
                    // Await all the futures
                    let results = join_all(volunteer_types_futures).await;
                    let mut collected_results = Vec::with_capacity(results.len());
                    for result in results {
                        match result {
                            Ok(music_genre) => collected_results.push(music_genre),
                            Err(e) => return Err(e), // Propagate the error if any
                        }
                    }
                    Set(collected_results)
                },
            notes: Set(self.notes.clone()),
            status: Set(
                self.status.to_sea_orm(db).await
                .map_err(|_| DbErr::AttrNotSet("status could not be set".to_string()))?
            ),            
            created_at: Set(self.created_at),
            updated_at: Set(self.updated_at),
        })
    }
}