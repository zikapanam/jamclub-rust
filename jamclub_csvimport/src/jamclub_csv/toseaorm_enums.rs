use super::ToSeaOrm;
use jamclub_seaorm::entities::sea_orm_active_enums::*;
use super::csv_enums::*;
use sea_orm::entity::prelude::*;
use sea_orm::DbErr;

// Implementation for CsvEncounterStatus
impl ToSeaOrm<EncounterStatus> for CsvEncounterStatus {
    async fn to_sea_orm(&self, _db : &DatabaseConnection) ->  Result<EncounterStatus, DbErr> {
        match self {
            CsvEncounterStatus::Cancelled => Ok(EncounterStatus::Cancelled),
            CsvEncounterStatus::ClosedRegistration => Ok(EncounterStatus::ClosedRegistration),
            CsvEncounterStatus::OnDemandeRegistration => Ok(EncounterStatus::OnDemandeRegistration),
            CsvEncounterStatus::OpenRegistration => Ok(EncounterStatus::OpenRegistration),
            CsvEncounterStatus::Realised => Ok(EncounterStatus::Realised),
        }
    }
}

// Implementation for CsvEncounterType
impl ToSeaOrm<EncounterType> for CsvEncounterType {
    async fn to_sea_orm(&self, _db : &DatabaseConnection) ->  Result<EncounterType, DbErr> {
        match self {
            CsvEncounterType::Concert => Ok(EncounterType::Concert),
            CsvEncounterType::DistantJam => Ok(EncounterType::DistantJam),
            CsvEncounterType::RehearsalJam => Ok(EncounterType::RehearsalJam),
            CsvEncounterType::OutJam => Ok(EncounterType::OutJam),
            CsvEncounterType::EventJam => Ok(EncounterType::EventJam),
            CsvEncounterType::StreetJam => Ok(EncounterType::StreetJam),
            CsvEncounterType::Karaoke => Ok(EncounterType::Karaoke),
            CsvEncounterType::Informal => Ok(EncounterType::Informal),            
        }
    }
}

// Implementation for CsvEventStatus
impl ToSeaOrm<EventStatus> for CsvEventStatus {
    async fn to_sea_orm(&self, _db : &DatabaseConnection) ->  Result<EventStatus, DbErr> {
        match self {
            CsvEventStatus::ToBook => Ok(EventStatus::ToBook),
            CsvEventStatus::Booked => Ok(EventStatus::Booked),
            CsvEventStatus::BookedAndConfirmed => Ok(EventStatus::BookedAndConfirmed),
            CsvEventStatus::Cancelled => Ok(EventStatus::Cancelled),
            CsvEventStatus::Realised => Ok(EventStatus::Realised),
        }
    }
}

// Implementation for CsvMusicGenre
impl ToSeaOrm<MusicGenre> for CsvMusicGenre {
    async fn to_sea_orm(&self, _db : &DatabaseConnection) ->  Result<MusicGenre, DbErr> {
        match self {
            CsvMusicGenre::Blues => Ok(MusicGenre::Blues),
            CsvMusicGenre::FrenchSong => Ok(MusicGenre::FrenchSong),
            CsvMusicGenre::Country => Ok(MusicGenre::Country),
            CsvMusicGenre::Disco => Ok(MusicGenre::Disco),
            CsvMusicGenre::Folk => Ok(MusicGenre::Folk),
            CsvMusicGenre::Funk => Ok(MusicGenre::Funk),
            CsvMusicGenre::Fusion => Ok(MusicGenre::Fusion),
            CsvMusicGenre::Grunge => Ok(MusicGenre::Grunge),
            CsvMusicGenre::HardRock => Ok(MusicGenre::HardRock),
            CsvMusicGenre::Instrumental => Ok(MusicGenre::Instrumental),
            CsvMusicGenre::Jazz => Ok(MusicGenre::Jazz),
            CsvMusicGenre::Latin => Ok(MusicGenre::Latin),
            CsvMusicGenre::Metal => Ok(MusicGenre::Metal),
            CsvMusicGenre::Pop => Ok(MusicGenre::Pop),
            CsvMusicGenre::Punk => Ok(MusicGenre::Punk),
            CsvMusicGenre::RnB => Ok(MusicGenre::RnB),
            CsvMusicGenre::Rock => Ok(MusicGenre::Rock),
            CsvMusicGenre::Soul => Ok(MusicGenre::Soul),
            CsvMusicGenre::World => Ok(MusicGenre::World),
            CsvMusicGenre::Reggae => Ok(MusicGenre::Reggae),
            CsvMusicGenre::Afrobeat => Ok(MusicGenre::Afrobeat),
            CsvMusicGenre::Brasilian => Ok(MusicGenre::Brasilian),
            CsvMusicGenre::HipHop => Ok(MusicGenre::HipHop),
            CsvMusicGenre::Trad => Ok(MusicGenre::Trad),
            CsvMusicGenre::Chamber => Ok(MusicGenre::Chamber),
        }
    }
}

// Implementation for CsvParticipantType
impl ToSeaOrm<ParticipantType> for CsvParticipantType {
    async fn to_sea_orm(&self, _db : &DatabaseConnection) ->  Result<ParticipantType, DbErr> {
        match self {
            CsvParticipantType::Available => Ok(ParticipantType::Available),
            CsvParticipantType::IfNeeded => Ok(ParticipantType::IfNeeded),
            CsvParticipantType::NotAvailable => Ok(ParticipantType::NotAvailable),
        }
    }
}

// Implementation for CsvPlayedInstrument
impl ToSeaOrm<PlayedInstrument> for CsvPlayedInstrument {
    async fn to_sea_orm(&self, _db : &DatabaseConnection) ->  Result<PlayedInstrument, DbErr> {
        match self {
            CsvPlayedInstrument::All => Ok(PlayedInstrument::All),
            CsvPlayedInstrument::Accordion => Ok(PlayedInstrument::Accordion),
            CsvPlayedInstrument::Bass => Ok(PlayedInstrument::Bass),
            CsvPlayedInstrument::Drums => Ok(PlayedInstrument::Drums),
            CsvPlayedInstrument::Cajon => Ok(PlayedInstrument::Cajon),
            CsvPlayedInstrument::Voice => Ok(PlayedInstrument::Voice),
            CsvPlayedInstrument::Clarinet => Ok(PlayedInstrument::Clarinet),
            CsvPlayedInstrument::Keyboard => Ok(PlayedInstrument::Keyboard),
            CsvPlayedInstrument::DoubleBass => Ok(PlayedInstrument::DoubleBass),
            CsvPlayedInstrument::DrumMachine => Ok(PlayedInstrument::DrumMachine),
            CsvPlayedInstrument::Flute => Ok(PlayedInstrument::Flute),
            CsvPlayedInstrument::Guitar => Ok(PlayedInstrument::Guitar),
            CsvPlayedInstrument::Harmonica => Ok(PlayedInstrument::Harmonica),
            CsvPlayedInstrument::CAM => Ok(PlayedInstrument::Cam),
            CsvPlayedInstrument::Percussion => Ok(PlayedInstrument::Percussion),
            CsvPlayedInstrument::Piano => Ok(PlayedInstrument::Piano),
            CsvPlayedInstrument::Saxophone => Ok(PlayedInstrument::Saxophone),
            CsvPlayedInstrument::Trombone => Ok(PlayedInstrument::Trombone),
            CsvPlayedInstrument::Trumpet => Ok(PlayedInstrument::Trumpet),
            CsvPlayedInstrument::Violin => Ok(PlayedInstrument::Violin),
            CsvPlayedInstrument::Harp => Ok(PlayedInstrument::Harp),
            CsvPlayedInstrument::Cello => Ok(PlayedInstrument::Cello),
            CsvPlayedInstrument::OtherStringedInstruments => Ok(PlayedInstrument::OtherStringedInstruments),
            CsvPlayedInstrument::OtherWoodwindInstruments => Ok(PlayedInstrument::OtherWoodwindInstruments),
        }
    }
}

// Implementation for CsvPublicationStatus
impl ToSeaOrm<PublicationStatus> for CsvPublicationStatus {
    async fn to_sea_orm(&self, _db : &DatabaseConnection) ->  Result<PublicationStatus, DbErr> {
        match self {
            CsvPublicationStatus::Archived => Ok(PublicationStatus::Archived),
            CsvPublicationStatus::Confirmed => Ok(PublicationStatus::Confirmed),
            CsvPublicationStatus::ToBeChecked => Ok(PublicationStatus::ToBeChecked),
        }
    }
}

// Implementation for CsvPublicationStatus
impl ToSeaOrm<PublicationStatus> for CsvArchivedStatus {
    async fn to_sea_orm(&self, _db : &DatabaseConnection) ->  Result<PublicationStatus, DbErr> {
        match self {
            CsvArchivedStatus::Archived => Ok(PublicationStatus::Archived),
            CsvArchivedStatus::Confirmed => Ok(PublicationStatus::Confirmed),
        }
    }
}


// Implementation for CsvZapRole
impl ToSeaOrm<ZapRole> for CsvZapRole {
    async fn to_sea_orm(&self, _db : &DatabaseConnection) ->  Result<ZapRole, DbErr> {
        match self {
            CsvZapRole::Recensement3 => Ok(ZapRole::Recensement3),
            CsvZapRole::MbrAsso => Ok(ZapRole::MbrAsso),
            CsvZapRole::MbrAssoActif => Ok(ZapRole::MbrAssoActif),
            CsvZapRole::MbrAssoBienfaiteur => Ok(ZapRole::MbrAssoBienfaiteur),
            CsvZapRole::Leader => Ok(ZapRole::Leader),
            CsvZapRole::Referent => Ok(ZapRole::Referent),
        }
    }
}

// Implementation for CsvProvenance
impl ToSeaOrm<Provenance> for CsvProvenance {
    async fn to_sea_orm(&self, _db : &DatabaseConnection) ->  Result<Provenance, DbErr> {
        match self {
            CsvProvenance::Basetorius => Ok(Provenance::Basetorius),
            CsvProvenance::Easyzic => Ok(Provenance::Easyzic),
            CsvProvenance::Flyers => Ok(Provenance::Flyers),
            CsvProvenance::Other => Ok(Provenance::Other),
            CsvProvenance::SearchEngine => Ok(Provenance::SearchEngine),
            CsvProvenance::SocialNetwork => Ok(Provenance::SocialNetwork),
            CsvProvenance::WordsOfMouth => Ok(Provenance::WordsOfMouth),
            CsvProvenance::Youtube => Ok(Provenance::Youtube),
            CsvProvenance::Zikinf => Ok(Provenance::Zikinf),
        }
    }
}


// Implementation for CsvLocationType
impl ToSeaOrm<LocationType> for CsvLocationType {
    async fn to_sea_orm(&self, _db : &DatabaseConnection) ->  Result<LocationType, DbErr> {
        match self {
            CsvLocationType::Cafe => Ok(LocationType::Cafe),
            CsvLocationType::CafeConcert => Ok(LocationType::CafeConcert),
            CsvLocationType::RehearsalStudio => Ok(LocationType::RehearsalStudio),
            CsvLocationType::ConcertHall => Ok(LocationType::ConcertHall),
            CsvLocationType::MeetingHall => Ok(LocationType::MeetingHall),
            CsvLocationType::Restaurant => Ok(LocationType::Restaurant),
            CsvLocationType::Karaoke => Ok(LocationType::Karaoke),
            CsvLocationType::PrivateLocation => Ok(LocationType::PrivateLocation),
        }
    }
}

// Implementation for CsvVolunteerType
impl ToSeaOrm<VolunteerType> for CsvVolunteerType {
    async fn to_sea_orm(&self, _db : &DatabaseConnection) ->  Result<VolunteerType, DbErr> {
        match self {
            CsvVolunteerType::ComputerScience => Ok(VolunteerType::ComputerScience),
            CsvVolunteerType::Designer => Ok(VolunteerType::Designer),
            CsvVolunteerType::Illustrator => Ok(VolunteerType::Illustrator),
            CsvVolunteerType::Photograph => Ok(VolunteerType::Photograph),
            CsvVolunteerType::Videographer => Ok(VolunteerType::Videographer),
        }
    }
}

// Implementation for CsvVolunteerType
impl ToSeaOrm<Community> for CsvCommunity {
    async fn to_sea_orm(&self, _db : &DatabaseConnection) -> Result<Community, DbErr> {
        match self {
            CsvCommunity::Interadv => Ok(Community::Interadv),
            CsvCommunity::Interbeg => Ok(Community::Interbeg),
        }
    }
}

// Implementation for CsvPublicationStatus
impl ToSeaOrm<EventType> for CsvEventType {
    async fn to_sea_orm(&self, _db : &DatabaseConnection) ->  Result<EventType, DbErr> {
        match self {
            CsvEventType::Concert => Ok(EventType::Concert),
            CsvEventType::OpenMic => Ok(EventType::OpenMic),
            CsvEventType::JamSession => Ok(EventType::JamSession),
        }
    }
}