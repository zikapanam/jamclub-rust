use sea_orm::DatabaseConnection;

use jamclub_seaorm::model_controller::{LineupManager, CollectiveManager, EncounterManager, EventManager};
use super::ToSeaOrm;
use super::csv_encounters::*;
use super::csv_collectives::*;
use super::csv_members::*;
use super::csv_locations::*;
use super::csv_events::*;
use super::csv_lineups::*;
use crate::error::error::MyError;
use std::fs::metadata;
use sea_orm::{Database,ActiveModelTrait};
use std::env;
use sea_orm::TryIntoModel;
use std::backtrace::Backtrace;
use log;

pub async fn run() -> Result<(), MyError> {
    let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let db = Database::connect(database_url.clone()).await?;    
    import_memebers(&db).await?;
    
    import_collectives(&db).await?;
    
    import_locations(&db).await?;

    import_encounters(&db).await?;

    import_lineups(&db).await?;

    import_events(&db).await?;
    Ok(())
}
pub async fn import_memebers(db : &DatabaseConnection) -> Result<(), MyError> {

    let args: Vec<String> = env::args().collect();
    
    if let Some(path) = args.get(1) {
        let md = metadata(path).unwrap();
        if md.is_dir() {


            log::trace!("Members ---");

            let mut rdr = csv::ReaderBuilder::new()
                    .has_headers(true)
                    .double_quote(true)
                    .delimiter(b';')
                    .from_path(format!("{}{}", path, "/membres.csv"))?;
            
            for record in rdr.deserialize() {
                let rec: CsvMembers = record?;
                let result = rec.to_sea_orm(db).await;
                match result {
                    Ok(active_model) => {active_model.save(db).await?;},
                    Err(e) => {log::error!("Error for members : [{}]\n{}\n{}", rec.ref_airtable.clone().unwrap(), e, Backtrace::capture());},
                }                
            }

            Ok(())

        } else {
            log::error!("{} is not a dir name.", path);
            panic!();
        }
    } else {
        log::error!("No dir name provided.");
        panic!();
    }    

}


pub async fn import_collectives(db : &DatabaseConnection) -> Result<(), MyError> {

    let args: Vec<String> = env::args().collect();
    
    if let Some(path) = args.get(1) {
        let md = metadata(path).unwrap();
        if md.is_dir() {
            
            log::trace!("Collectives ---");
            let mut rdr = csv::ReaderBuilder::new()
                    .has_headers(true)
                    .double_quote(true)
                    .delimiter(b';')
                    .from_path(format!("{}{}", path, "/collectifs.csv"))?;

            for record in rdr.deserialize() {
                let record: CsvCollectives = record?;
                let collective = record.to_sea_orm(db).await;
                match collective {
                    Ok(coll) => {
                        let c = coll.save(db).await?.try_into_model()?;
                        if record.referent_airtable_ids.trim() != "" {
                            let rref_ids = record.referent_airtable_ids.split(',').map(|e| e.trim().to_string()).collect::<Vec<String>>();
                            let referents = 
                                CollectiveManager::addall_referents_by_airtable_ref(
                                    c.clone(), 
                                    Some(rref_ids), 
                                    db).await;
                            if let Err(e) = referents {
                                log::error!("Error for collective (referent) : [{}]\n{}\n{}", c.ref_airtable.clone().unwrap(), e, Backtrace::capture());
                            }
                        }                        
                        if record.subscriber_airtable_ids.trim() != "" {
                            let sref_ids = record.subscriber_airtable_ids.split(',').map(|e| e.trim().to_string()).collect::<Vec<String>>();
                            let subscribers = 
                                    CollectiveManager::addall_subscribers_by_airtable_ref(
                                        c.clone(), 
                                        Some(sref_ids), 
                                        db).await;
                            if let Err(e) = subscribers {
                                log::error!("Error for collective (subscriber) : [{}]\n{}\n{}", c.ref_airtable.clone().unwrap(), e, Backtrace::capture());
                            }
                        }
                    },
                    Err(e) => {
                        log::error!("Error for collectives : {}", e);
                    }
                }
            }

            Ok(())

        } else {
            log::error!("{} is not a dir name.", path);
            panic!();
        }
    } else {
        log::error!("No dir name provided.");
        panic!();
    }    

}

pub async fn import_encounters(db : &DatabaseConnection) -> Result<(), MyError> {

    let args: Vec<String> = env::args().collect();
    
    if let Some(path) = args.get(1) {
        let md = metadata(path).unwrap();
        if md.is_dir() {

            log::trace!("Encounters ---");
            let mut rdr = csv::ReaderBuilder::new()
                    .has_headers(true)
                    .double_quote(true)
                    .delimiter(b';')
                    .from_path(format!("{}{}", path, "/rencontres.csv"))?;

            for record in rdr.deserialize() {
                let record: CsvEncounters = record?;
                let encounter = record.to_sea_orm(db).await;
                match encounter {
                    Ok(enc) => {
                        let c = enc.save(db).await?.try_into_model()?;
                        if record.absent_airtable_ids.trim() != "" {
                            let rref_ids = record.absent_airtable_ids.split(',').map(|e| e.trim().to_string()).collect::<Vec<String>>();
                            let absents = 
                                EncounterManager::addall_absents_by_airtable_ref(
                                    c.clone(), 
                                    Some(rref_ids), 
                                    db).await;
                            if let Err(e) = absents {
                                log::error!("Error for encounter (absent) : [{}]\n{}\n{}", c.ref_airtable.clone().unwrap(), e, Backtrace::capture());
                            }
                        }                        
                        if record.participant_airtable_ids.trim() != "" {
                            let sref_ids = record.participant_airtable_ids.split(',').map(|e| e.trim().to_string()).collect::<Vec<String>>();
                            let participants = 
                                    EncounterManager::addall_participant_by_airtable_ref(
                                        c.clone(), 
                                        Some(sref_ids), 
                                        db).await;
                            if let Err(e) = participants {
                                log::error!("Error for encounter (participant) : [{}]\n{}\n{}", c.ref_airtable.clone().unwrap(), e, Backtrace::capture());
                            }
                        }
                        if record.participant_if_needed_airtable_ids.trim() != "" {
                            let sref_ids = record.participant_if_needed_airtable_ids.split(',').map(|e| e.trim().to_string()).collect::<Vec<String>>();
                            let participants_if_needed = 
                                    EncounterManager::addall_participant_if_needed_by_airtable_ref(
                                        c.clone(), 
                                        Some(sref_ids), 
                                        db).await;
                            if let Err(e) = participants_if_needed {
                                log::error!("Error for encounter (participant if needed) : [{}]\n{}\n{}", c.ref_airtable.clone().unwrap(), e, Backtrace::capture());
                            }
                        }
                        if record.participant_not_available_airtable_ids.trim() != "" {
                            let sref_ids = record.participant_not_available_airtable_ids.split(',').map(|e| e.trim().to_string()).collect::<Vec<String>>();
                            let participants_not_available = 
                                    EncounterManager::addall_participant_not_available_by_airtable_ref(
                                        c.clone(), 
                                        Some(sref_ids), 
                                        db).await;
                            if let Err(e) = participants_not_available {
                                log::error!("Error for encounter (participant not available) : [{}]\n{}\n{}", c.ref_airtable.clone().unwrap(), e, Backtrace::capture());
                            }
                        }
                    },
                    Err(e) => {
                        log::error!("Error for collectives : {}", e);
                    }
                }

            }
            Ok(())
        } else {
            log::error!("{} is not a dir name.", path);
            panic!();
        }
    } else {
        log::error!("No dir name provided.");
        panic!();
    }    
}

pub async fn import_locations(db : &DatabaseConnection) -> Result<(), MyError> {

    let args: Vec<String> = env::args().collect();
    
    if let Some(path) = args.get(1) {
        let md = metadata(path).unwrap();
        if md.is_dir() {

            log::trace!("Locations ---");
            let rdr = csv::ReaderBuilder::new()
                    .has_headers(true)
                    .double_quote(true)
                    .delimiter(b';')
                    .from_path(format!("{}{}", path, "/lieux.csv"));

            let mut r = rdr?;

            for record in r.deserialize() {
                match record {
                    Ok(r) => {
                        let rec: CsvLocations = r;
                        let result = rec.to_sea_orm(db).await;
                        match result {
                            Ok(active_model) => {active_model.save(db).await?;},
                            Err(e) => log::error!("Error for locations: [{}]\n{}\n{}", rec.ref_airtable.clone().unwrap(), e, Backtrace::capture()),  
                        }
                    },
                    Err(e) => {log::error!("Error for locations : {}\n{}", e, Backtrace::capture());},

                }
            }

            Ok(())

        } else {
            log::error!("{} is not a dir name.", path);
            panic!();
        }
    } else {
        log::error!("No dir name provided.");
        panic!();
    }    

}

pub async fn import_events(db : &DatabaseConnection) -> Result<(), MyError> {

    let args: Vec<String> = env::args().collect();
    
    if let Some(path) = args.get(1) {
        let md = metadata(path).unwrap();
        if md.is_dir() {

            log::trace!("Events ---");
            let rdr = csv::ReaderBuilder::new()
                    .has_headers(true)
                    .double_quote(true)
                    .delimiter(b';')
                    .from_path(format!("{}{}", path, "/evenements.csv"));

            let mut r = rdr?;

            for record in r.deserialize() {
                let record: CsvEvents = record?;
                let event = record.to_sea_orm(db).await;
                match event {
                    Ok(ev) => {
                        let c = ev.save(db).await?.try_into_model()?;
                        if record.lineup_airtable_ids.trim() != "" {
                            let sref_ids = record.lineup_airtable_ids.split(',').map(|e| e.trim().to_string()).collect::<Vec<String>>();
                            let event_lineup = 
                                    EventManager::addall_lineup_by_airtable_ref(
                                        c.clone(), 
                                        Some(sref_ids), 
                                        db).await;
                            if let Err(e) = event_lineup {
                                log::error!("Error for lineup (subscriber) : [{}]\n{}\n{}", c.ref_airtable.clone().unwrap(), e, Backtrace::capture());
                            }
                        }
                        // if record.volunteer_airtable_ids.trim() != "" {
                        //     let sref_ids = record.subscriber_airtable_ids.split(',').map(|e| e.trim().to_string()).collect::<Vec<String>>();
                        //     let lineup_subscriber = 
                        //             LineupManager::addall_subscribers_by_airtable_ref(
                        //                 c.clone(), 
                        //                 Some(sref_ids), 
                        //                 db).await;
                        //     if let Err(e) = lineup_subscriber {
                        //         println!("Error for lineups (subscribers) : {}\n{}", e, Backtrace::capture());
                        //     }
                        // }
                    },
                    Err(e) => {
                        log::error!("Error for collectives : {}", e);
                    }
                }

            }
            Ok(())
        
        } else {
            log::error!("{} is not a dir name.", path);
            panic!();
        }
    } else {
        log::error!("No dir name provided.");
        panic!();
    }    

}


pub async fn import_lineups(db : &DatabaseConnection) -> Result<(), MyError> {

    let args: Vec<String> = env::args().collect();
    
    if let Some(path) = args.get(1) {
        let md = metadata(path).unwrap();
        if md.is_dir() {

            log::trace!("Lineups ---");
            let rdr = csv::ReaderBuilder::new()
                    .has_headers(true)
                    .double_quote(true)
                    .delimiter(b';')
                    .from_path(format!("{}{}", path, "/lineups.csv"));

            let mut r = rdr?;

            for record in r.deserialize() {
                let record: CsvLineups = record?;
                let lineup = record.to_sea_orm(db).await;
                match lineup {
                    Ok(lin) => {
                        let c = lin.save(db).await?.try_into_model()?;
                        if record.subscriber_airtable_ids.trim() != "" {
                            let sref_ids = record.subscriber_airtable_ids.split(',').map(|e| e.trim().to_string()).collect::<Vec<String>>();
                            let lineup_subscriber = 
                                    LineupManager::addall_subscribers_by_airtable_ref(
                                        c.clone(), 
                                        Some(sref_ids), 
                                        db).await;
                            if let Err(e) = lineup_subscriber {
                                log::error!("Error for lineups (subscribers) : [{}]\n{}\n{}", c.ref_airtable.clone().unwrap(), e, Backtrace::capture());
                            }
                        }
                    },
                    Err(e) => {
                        log::error!("Error for collectives : {}", e);
                    }
                }

            }
            Ok(())

        } else {
            log::error!("{} is not a dir name.", path);
            panic!();
        }
    } else {
        log::error!("No dir name provided.");
        panic!();
    }    

}

