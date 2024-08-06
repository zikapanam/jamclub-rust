use sea_orm::{entity::*, error::*, query::*, DatabaseConnection};
use crate::entities::sea_orm_active_enums::Community;
use crate::entities::sea_orm_active_enums::EncounterStatus;

use super::entities::prelude::*;
use super::entities::members;
use super::entities::locations;
use super::entities::lineups;
use super::entities::collectives;
use super::entities::encounters;
use super::entities::events;
use super::entities::collectives_referents;
use super::entities::collectives_subscribers;
use super::entities::lineups_subscribers;
use super::entities::encounters_absents;
use super::entities::encounters_participants;
use super::entities::events_lineups;
use super::entities::sea_orm_active_enums::ParticipantType;
use super::entities::sea_orm_active_enums::EventStatus;
use futures::future::join_all;
use chrono::prelude::*;

use log;

pub struct MemberManager {}



impl MemberManager {

    pub async fn find_by_airtable_ref(member_ref_airtable: String, db :&DatabaseConnection) -> Result<Option<members::Model>, sea_orm::DbErr> {
        let member : Option<members::Model> = Members::find()
        .filter(members::Column::RefAirtable.eq(member_ref_airtable))
        .one(db)
        .await?;

        Ok(member)
    }

    pub async fn find_by_nickname(member_nickname: String, db :&DatabaseConnection) -> Result<Option<members::Model>, sea_orm::DbErr> {
        let member : Option<members::Model> = Members::find()
        .filter(members::Column::NickName.eq(member_nickname))
        .one(db)
        .await?;

        Ok(member)
    }


}

pub struct LocationManager {}


impl LocationManager {

    pub async fn find_by_airtable_ref(location_ref_airtable: String, db :&DatabaseConnection) -> Result<Option<locations::Model>, sea_orm::DbErr> {
        let location : Option<locations::Model> = Locations::find()
        .filter(locations::Column::RefAirtable.eq(location_ref_airtable))
        .one(db)
        .await?;
        Ok(location)
    }

}

pub struct CollectiveManager {}


impl CollectiveManager {

    pub async fn find_by_airtable_ref(collective_ref_airtable: String, db :&DatabaseConnection) -> Result<Option<collectives::Model>, sea_orm::DbErr> {
        let collective : Option<collectives::Model> = Collectives::find()
        .filter(collectives::Column::RefAirtable.eq(collective_ref_airtable))
        .one(db)
        .await?;
        Ok(collective)
    }

    pub async fn addall_referents_by_airtable_ref(collective : collectives::Model, referent_airtable_ids : Option<Vec<String>>, db :&DatabaseConnection) -> Result<Option<Vec<collectives_referents::Model>>, sea_orm::DbErr> {
        if let Some(ids) = referent_airtable_ids {
            collectives_referents::Entity::delete_many()
            .filter(collectives_referents::Column::CollectiveId.eq(collective.id))
            .exec(db)
            .await?;
        
            let referent_futures = ids
                .into_iter()
                .map(|id| MemberManager::find_by_airtable_ref(id.trim().to_string(), db))
                .collect::<Vec<_>>();
    
            let results = join_all(referent_futures).await;
    
            let mut collected_results = Vec::with_capacity(results.len());
            for result in results {

                match result {
                    Ok(Some(referent)) => {
                        log::info!("referent: {}, collective: {}, created_at: {}", referent.nick_name.unwrap().clone(), collective.short_title.clone(), collective.updated_at.clone());
                        
                        let collective_referent = collectives_referents::ActiveModel {
                            referent_id: Set(referent.id.clone()),
                            collective_id: Set(collective.id.clone()),
                            created_at: Set(collective.updated_at.clone()),
                        };
                        
                        let cam = collective_referent.insert(db).await?;
                        let cm = cam.try_into_model()?;
                        log::info!("model: collective_id: {}, referent_id: {}\n", cm.collective_id, cm.referent_id);
                        collected_results.push(cm);
                    }
                    Ok(None) => (),
                    Err(e) => {
                        return Err(DbErr::Custom(format!("Error on referent {} : {}", collective.id, e)));
                    },
                }
            }
            Ok(Some(collected_results))
        } else {
            Ok(None)
        }
    }

    pub async fn addall_subscribers_by_airtable_ref(collective : collectives::Model, subscriber_airtable_ids : Option<Vec<String>>, db :&DatabaseConnection) -> Result<Option<Vec<collectives_subscribers::Model>>, sea_orm::DbErr> {
        if let Some(ids) = subscriber_airtable_ids {
            collectives_subscribers::Entity::delete_many()
            .filter(collectives_subscribers::Column::CollectiveId.eq(collective.id))
            .exec(db)
            .await?;

            let subscriber_futures = ids
                .into_iter()
                .map(|id| MemberManager::find_by_airtable_ref(id, db))
                .collect::<Vec<_>>();
    
            let results = join_all(subscriber_futures).await;
    
            let mut collected_results = Vec::with_capacity(results.len());
            for result in results {
                match result {
                    Ok(Some(subscriber)) => {
                        log::info!("subscriber: {}, collective: {}, created_at: {}", subscriber.nick_name.unwrap().clone(), collective.short_title.clone(), collective.updated_at.clone());
                        let collective_subscriber = collectives_subscribers::ActiveModel {
                            subscriber_id: Set(subscriber.id.clone()),
                            collective_id: Set(collective.id.clone()),
                            created_at: Set(collective.updated_at.clone()),
                        };
                        let cam = collective_subscriber.insert(db).await?;
                        let cm = cam.try_into_model()?;
                        log::info!("model: collective_id: {}, subscriber_id: {}\n", cm.collective_id, cm.subscriber_id);
                        collected_results.push(cm);
                    },
                    Ok(None) => (),
                    Err(e) => {
                        return Err(DbErr::Custom(format!("Error on subscriber {} : {}", collective.id, e)));
                    },
                }
            }
            Ok(Some(collected_results))
        } else {
            Ok(None)
        }
    }  
}  

pub struct LineupManager {}


impl LineupManager {

    pub async fn find_by_airtable_ref(lineup_ref_airtable: String, db :&DatabaseConnection) -> Result<Option<lineups::Model>, sea_orm::DbErr> {
        let lineup : Option<lineups::Model> = Lineups::find()
        .filter(lineups::Column::RefAirtable.eq(lineup_ref_airtable))
        .one(db)
        .await?;
        Ok(lineup)
    }

    pub async fn addall_subscribers_by_airtable_ref(lineup : lineups::Model, subscriber_airtable_ids : Option<Vec<String>>, db :&DatabaseConnection) -> Result<Option<Vec<lineups_subscribers::Model>>, sea_orm::DbErr> {
        if let Some(ids) = subscriber_airtable_ids {
            lineups_subscribers::Entity::delete_many()
            .filter(lineups_subscribers::Column::LineupId.eq(lineup.id))
            .exec(db)
            .await?;

            let subscriber_futures = ids
                .into_iter()
                .map(|id| MemberManager::find_by_airtable_ref(id, db))
                .collect::<Vec<_>>();
    
            let results = join_all(subscriber_futures).await;
    
            let mut collected_results = Vec::with_capacity(results.len());
            for result in results {
                match result {
                    Ok(Some(subscriber)) => {
                        log::info!("subscriber: {}, lineup: {}, created_at: {}", subscriber.nick_name.unwrap().clone(), lineup.short_title.clone(), lineup.updated_at.clone());
                        let lineup_subscriber = lineups_subscribers::ActiveModel {
                            subscriber_id: Set(subscriber.id.clone()),
                            lineup_id: Set(lineup.id.clone()),
                            created_at: Set(lineup.updated_at.clone()),
                        };
                        let cam = lineup_subscriber.insert(db).await?;
                        let cm = cam.try_into_model()?;
                        log::info!("model: lineup_id: {}, subscriber_id: {}\n", cm.lineup_id, cm.subscriber_id);
                        collected_results.push(cm);
                    },
                    Ok(None) => (),
                    Err(e) => {
                        return Err(DbErr::Custom(format!("Error on subscriber {} : {}", lineup.id, e)));
                    },
                }
            }
            Ok(Some(collected_results))
        } else {
            Ok(None)
        }
    }    

}

pub struct EncounterManager {}


impl EncounterManager {

    pub async fn find_by_airtable_ref(encounter_ref_airtable: String, db :&DatabaseConnection) -> Result<Option<encounters::Model>, sea_orm::DbErr> {
        let encounter : Option<encounters::Model> = Encounters::find()
        .filter(encounters::Column::RefAirtable.eq(encounter_ref_airtable))
        .one(db)
        .await?;
        Ok(encounter)
    }

    pub async fn find_encounters_to_come_and_to_fill_in_by_communities(communities: Vec<Community>, limit : u64, db :&DatabaseConnection) -> Result<Vec<encounters::Model>, sea_orm::DbErr> {
        Encounters::find()
        .filter(Condition::all().add(encounters::Column::StartDate.gte(Utc::now()))
                                .add(Condition::any().add(encounters::Column::Communities.eq(communities)))
                                .add(encounters::Column::Status.is_in(vec![EncounterStatus::OpenRegistration, EncounterStatus::OnDemandeRegistration]))
        )
        .order_by_asc(encounters::Column::StartDate)
        .limit(limit)
        .all(db)
        .await
    }

    pub async fn get_location(encounter: encounters::Model, db : &DatabaseConnection) -> Result<Option<locations::Model>, sea_orm::DbErr> {
        encounter.find_related(Locations).one(db).await
    }

    pub async fn get_collective(encounter: encounters::Model, db : &DatabaseConnection) -> Result<Option<collectives::Model>, sea_orm::DbErr> {
        encounter.find_related(Collectives).one(db).await
    }

    pub async fn addall_absents_by_airtable_ref(encounter : encounters::Model, referent_airtable_ids : Option<Vec<String>>, db :&DatabaseConnection) -> Result<Option<Vec<encounters_absents::Model>>, sea_orm::DbErr> {
        if let Some(ids) = referent_airtable_ids {
            encounters_absents::Entity::delete_many()
            .filter(encounters_absents::Column::EncounterId.eq(encounter.id))
            .exec(db)
            .await?;

            let referent_futures = ids
                .into_iter()
                .map(|id| MemberManager::find_by_airtable_ref(id.trim().to_string(), db))
                .collect::<Vec<_>>();
    
            let results = join_all(referent_futures).await;
    
            let mut collected_results = Vec::with_capacity(results.len());
            for result in results {

                match result {
                    Ok(Some(absent)) => {
                        log::info!("absent: {}, encounter: {}, created_at: {}", absent.nick_name.unwrap().clone(), encounter.long_title.clone(), encounter.updated_at.clone());
                        let encounter_absent = encounters_absents::ActiveModel {
                            encounter_id: Set(encounter.id.clone()),
                            absent_id: Set(absent.id.clone()),
                            created_at: Set(encounter.updated_at.clone()),
                        };
                        let cam = encounter_absent.insert(db).await?;
                        let cm = cam.try_into_model()?;
                        log::info!("model: encounter_id: {}, absent_id: {}\n", cm.encounter_id, cm.absent_id);
                        collected_results.push(cm);
                    }
                    Ok(None) => (),
                    Err(e) => {
                        return Err(DbErr::Custom(format!("Error on referent {} : {}", encounter.id, e)));
                    },
                }
            }
            Ok(Some(collected_results))
        } else {
            Ok(None)
        }
    }    

    pub async fn addall_participant_by_airtable_ref(encounter : encounters::Model, referent_airtable_ids : Option<Vec<String>>, db :&DatabaseConnection) -> Result<Option<Vec<encounters_participants::Model>>, sea_orm::DbErr> {
        if let Some(ids) = referent_airtable_ids {
            encounters_participants::Entity::delete_many()
            .filter(Condition::all().add(encounters_participants::Column::EncounterId.eq(encounter.id))
                                    .add(encounters_participants::Column::ParticipantType.eq(ParticipantType::Available))
            )
            .exec(db)
            .await?;

            let referent_futures = ids
                .into_iter()
                .map(|id| MemberManager::find_by_airtable_ref(id.trim().to_string(), db))
                .collect::<Vec<_>>();
    
            let results = join_all(referent_futures).await;
    
            let mut collected_results = Vec::with_capacity(results.len());
            for result in results {

                match result {
                    Ok(Some(participant)) => {
                        log::info!("participant: {}, encounter: {}, created_at: {}", participant.nick_name.unwrap().clone(), encounter.long_title.clone(), encounter.updated_at.clone());
                        let encounter_participant = encounters_participants::ActiveModel {
                            encounter_id: Set(encounter.id.clone()),
                            participant_id: Set(participant.id.clone()),
                            participant_type: Set(ParticipantType::Available),
                            created_at: Set(encounter.updated_at.clone()),
                        };
                        let cam = encounter_participant.insert(db).await?;
                        let cm = cam.try_into_model()?;
                        log::info!("model: encounter_id: {}, participant_id: {}\n", cm.encounter_id, cm.participant_id);
                        collected_results.push(cm);
                    }
                    Ok(None) => (),
                    Err(e) => {
                        return Err(DbErr::Custom(format!("Error on referent {} : {}", encounter.id, e)));
                    },
                }
            }
            Ok(Some(collected_results))
        } else {
            Ok(None)
        }
    }    

    pub async fn addall_participant_not_available_by_airtable_ref(encounter : encounters::Model, referent_airtable_ids : Option<Vec<String>>, db :&DatabaseConnection) -> Result<Option<Vec<encounters_participants::Model>>, sea_orm::DbErr> {
        if let Some(ids) = referent_airtable_ids {
            encounters_participants::Entity::delete_many()
            .filter(Condition::all().add(encounters_participants::Column::EncounterId.eq(encounter.id))
                                    .add(encounters_participants::Column::ParticipantType.eq(ParticipantType::NotAvailable))
            )
            .exec(db)
            .await?;

            let referent_futures = ids
                .into_iter()
                .map(|id| MemberManager::find_by_airtable_ref(id.trim().to_string(), db))
                .collect::<Vec<_>>();

            let results = join_all(referent_futures).await;

            let mut collected_results = Vec::with_capacity(results.len());
            for result in results {

                match result {
                    Ok(Some(participant_not_available)) => {
                        log::info!("participant_not_available: {}, encounter: {}, created_at: {}", participant_not_available.nick_name.unwrap().clone(), encounter.long_title.clone(), encounter.updated_at.clone());
                        let encounter_participant = encounters_participants::ActiveModel {
                            encounter_id: Set(encounter.id.clone()),
                            participant_id: Set(participant_not_available.id.clone()),
                            participant_type: Set(ParticipantType::NotAvailable),
                            created_at: Set(encounter.updated_at.clone()),
                        };
                    let cam = encounter_participant.insert(db).await?;
                        let cm = cam.try_into_model()?;
                        log::info!("model: encounter_id: {}, participant_id: {}\n", cm.encounter_id, cm.participant_id);
                        collected_results.push(cm);
                    }
                    Ok(None) => (),
                    Err(e) => {
                        return Err(DbErr::Custom(format!("Error on referent {} : {}", encounter.id, e)));
                    },
                }
            }
            Ok(Some(collected_results))
        } else {
            Ok(None)
        }
    }    

    pub async fn addall_participant_if_needed_by_airtable_ref(encounter : encounters::Model, referent_airtable_ids : Option<Vec<String>>, db :&DatabaseConnection) -> Result<Option<Vec<encounters_participants::Model>>, sea_orm::DbErr> {
        if let Some(ids) = referent_airtable_ids {
            encounters_participants::Entity::delete_many()
            .filter(Condition::all().add(encounters_participants::Column::EncounterId.eq(encounter.id))
                                    .add(encounters_participants::Column::ParticipantType.eq(ParticipantType::IfNeeded))
            )
            .exec(db)
            .await?;

            let referent_futures = ids
                .into_iter()
                .map(|id| MemberManager::find_by_airtable_ref(id.trim().to_string(), db))
                .collect::<Vec<_>>();

            let results = join_all(referent_futures).await;

            let mut collected_results = Vec::with_capacity(results.len());
            for result in results {

                match result {
                    Ok(Some(participant_if_needed)) => {
                        log::info!("participant_if_needed: {}, encounter: {}, created_at: {}", participant_if_needed.nick_name.unwrap().clone(), encounter.long_title.clone(), encounter.updated_at.clone());
                        let encounter_participant = encounters_participants::ActiveModel {
                            encounter_id: Set(encounter.id.clone()),
                            participant_id: Set(participant_if_needed.id.clone()),
                            participant_type: Set(ParticipantType::IfNeeded),
                            created_at: Set(encounter.updated_at.clone()),
                        };
                    let cam = encounter_participant.insert(db).await?;
                        let cm = cam.try_into_model()?;
                        log::info!("model: encounter_id: {}, participant_id: {}\n", cm.encounter_id, cm.participant_id);
                        collected_results.push(cm);
                    }
                    Ok(None) => (),
                    Err(e) => {
                        return Err(DbErr::Custom(format!("Error on referent {} : {}", encounter.id, e)));
                    },
                }
            }
            Ok(Some(collected_results))
        } else {
            Ok(None)
        }
    }    


}


pub struct EventManager {}


impl EventManager {

    pub async fn find_by_airtable_ref(event_ref_airtable: String, db :&DatabaseConnection) -> Result<Option<events::Model>, sea_orm::DbErr> {
        let event : Option<events::Model> = Events::find()
        .filter(events::Column::RefAirtable.eq(event_ref_airtable))
        .one(db)
        .await?;
        Ok(event)
    }

    pub async fn find_events_to_come(limit : u64, db :&DatabaseConnection) -> Result<Vec<events::Model>, sea_orm::DbErr> {
        Events::find()
        .filter(Condition::all().add(events::Column::StartDate.gte(Utc::now()))
                                .add(events::Column::Status.eq(EventStatus::BookedAndConfirmed))
        )
        .order_by_asc(events::Column::StartDate)
        .limit(limit)
        .all(db)
        .await    
    }

    pub async fn get_location(event: events::Model, db : &DatabaseConnection) -> Result<Option<locations::Model>, sea_orm::DbErr> {
        event.find_related(Locations).one(db).await
    }

    pub async fn addall_lineup_by_airtable_ref(event : events::Model, lineup_airtable_ids : Option<Vec<String>>, db :&DatabaseConnection) -> Result<Option<Vec<events_lineups::Model>>, sea_orm::DbErr> {
        if let Some(ids) = lineup_airtable_ids {
            events_lineups::Entity::delete_many()
            .filter(events_lineups::Column::EventId.eq(event.id))
            .exec(db)
            .await?;


            let lineup_futures = ids
                .into_iter()
                .map(|id| LineupManager::find_by_airtable_ref(id.trim().to_string(), db))
                .collect::<Vec<_>>();
    
            let results = join_all(lineup_futures).await;
    
            let mut collected_results = Vec::with_capacity(results.len());
            for result in results {

                match result {
                    Ok(Some(lineup)) => {
                        log::info!("lineup: {}, event: {}, created_at: {}", lineup.short_title.clone(), event.title.clone(), event.updated_at.clone());
                        let event_lineup = events_lineups::ActiveModel {
                            event_id: Set(event.id.clone()),
                            lineup_id: Set(lineup.id.clone()),
                            created_at: Set(event.updated_at.clone()),
                        };
                        let cam = event_lineup.insert(db).await?;
                        let cm = cam.try_into_model()?;
                        log::info!("model: event_id: {}, lineup_id: {}\n", cm.event_id, cm.lineup_id);
                        collected_results.push(cm);
                    }
                    Ok(None) => (),
                    Err(e) => {
                        return Err(DbErr::Custom(format!("Error on referent {} : {}", event.id, e)));
                    },
                }
            }
            Ok(Some(collected_results))
        } else {
            Ok(None)
        }
    }    

    // pub async fn addall_volunteers_by_name(event : events::Model, volunteer_names : Option<Vec<String>>, db :&DatabaseConnection) -> Result<Option<Vec<events_volunteers::Model>>, sea_orm::DbErr> {
    //     if let Some(names) = volunteer_names {
    //         let Volunteers_futures = names
    //             .into_iter()
    //             .map(|id| MemberManager::find_by_airtable_ref(id, db))
    //             .collect::<Vec<_>>();
    
    //         let results = join_all(Volunteers_futures).await;
    
    //         let mut collected_results = Vec::with_capacity(results.len());
    //         for result in results {
    //             match result {
    //                 Ok(Some(volunteer)) => {
    //                     println!("volunteer: {}, event: {}, created_at: {}", volunteer.name.clone(), event.title.clone(), event.updated_at.clone());
    //                     let event_volunteer = collectives_subscribers::ActiveModel {
    //                         subscriber_id: Set(volunteer.id.clone()),
    //                         collective_id: Set(event.id.clone()),
    //                         created_at: Set(event.updated_at.clone()),
    //                     };
    //                     let cam = event_volunteer.insert(db).await?;
    //                     let cm = cam.try_into_model()?;
    //                     println!("model: event_id: {}, volunteer_id: {}\n", cm.event_id, cm.volunteer_id);
    //                     collected_results.push(cm);
    //                 },
    //                 Ok(None) => (),
    //                 Err(e) => {
    //                     return Err(DbErr::Custom(format!("Error on subscriber {} : {}", event.id, e)));
    //                 },
    //             }
    //         }
    //         Ok(Some(collected_results))
    //     } else {
    //         Ok(None)
    //     }
    // }    

}