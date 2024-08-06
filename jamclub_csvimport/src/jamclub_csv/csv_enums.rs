use std::str::FromStr;
use std::fmt;

#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum CsvEncounterStatus {
    #[serde(alias="Annulée")]
    Cancelled,
    #[serde(alias="inscription close")]
    ClosedRegistration,
    #[serde(alias="Inscription sur demande")]
    OnDemandeRegistration,
    #[serde(alias="Inscription ouverte")]
    OpenRegistration,
    #[serde(alias="Réalisée")]
    Realised,
}
#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum CsvEncounterType {
    #[serde(alias="ok-jams-concerts", alias="Concert")]
    Concert,
    #[serde(alias="ok-jamsdistancielles", alias="Jam distancielle (Par internet)")]
    DistantJam,
    #[serde(alias="ok-jamostudios", alias="Jam-O-Studio")]
    RehearsalJam,
    #[serde(alias="ok-sortiejams", alias="Sorties jams (scène ouverte | Jam session)")]
    OutJam,
    #[serde(alias="ok-soireejams", alias="Soirée jams (Organisée par l'association)")]
    EventJam,
    #[serde(alias="ok-streetjams", alias="Street Jam")]
    StreetJam,
    #[serde(alias="ok-karaokes", alias="Karaoké")]
    Karaoke,
    #[serde(alias="ok-informelle", alias="Rencontre sans jouer de la musique")]
    Informal,
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum CsvEventStatus {
    #[serde(alias="à booker")]
    ToBook,
    #[serde(alias="booké")]
    Booked,
    #[serde(alias="confirmé et booké")]
    BookedAndConfirmed,
    #[serde(alias="annulé")]
    Cancelled,
    #[serde(alias="réalisé")]
    Realised,
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum CsvMusicGenre {
    #[serde(alias="Blues")]
    Blues,
    #[serde(alias="Chanson française")]
    FrenchSong,
    #[serde(alias="Country")]
    Country,
    #[serde(alias="Disco")]
    Disco,
    #[serde(alias="Folk")]
    Folk,
    #[serde(alias="Funk")]
    Funk,
    #[serde(alias="Fusion")]
    Fusion,
    #[serde(alias="Grunge")]
    Grunge,
    #[serde(alias="Hard Rock")]
    HardRock,
    #[serde(alias="Instrumental")]
    Instrumental,
    #[serde(alias="Jazz")]
    Jazz,
    #[serde(alias="Latin")]
    Latin,
    #[serde(alias="Metal")]
    Metal,
    #[serde(alias="Pop")]
    Pop,
    #[serde(alias="Punk")]
    Punk,
    #[serde(alias="RnB")]
    RnB,
    #[serde(alias="Rock")]
    Rock,
    #[serde(alias="Soul")]
    Soul,
    #[serde(alias="World")]
    World,
    #[serde(alias="Reggae")]
    Reggae,
    #[serde(alias="Afrobeat")]
    Afrobeat,
    #[serde(alias="Brésilien")]
    Brasilian,
    #[serde(alias="Hip Hop")]
    HipHop,
    #[serde(alias="Trad")]
    Trad,
    #[serde(alias="De chambre")]
    Chamber,
    
        
}
#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum CsvParticipantType {
    Available,
    IfNeeded,
    NotAvailable,
}
#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum CsvPlayedInstrument {
    #[serde(alias="Tout Type d'Instrument")]
    All,
    #[serde(alias="Accordéon")]
    Accordion,
    #[serde(alias="Basse")]
    Bass,
    #[serde(alias="Batterie")]
    Drums,
    #[serde(alias="Cajon")]
    Cajon,
    #[serde(alias="Chant")]
    Voice,
    #[serde(alias="Clarinette")]
    Clarinet,
    #[serde(alias="Clavier")]
    Keyboard,
    #[serde(alias="Contrebasse")]
    DoubleBass,
    #[serde(alias="Drum machine")]
    DrumMachine,
    #[serde(alias="Flûte")]
    Flute,
    #[serde(alias="Guitare")]
    Guitar,
    #[serde(alias="Harmonica")]
    Harmonica,
    #[serde(alias="MAO")]
    CAM,
    #[serde(alias="Percussions")]
    Percussion,
    #[serde(alias="Piano")]
    Piano,
    #[serde(alias="Saxophone")]
    Saxophone,
    #[serde(alias="Trombone")]
    Trombone,
    #[serde(alias="Trompette")]
    Trumpet,
    #[serde(alias="Violon")]
    Violin,
    #[serde(alias="Harpe")]
    Harp,
    #[serde(alias="+Violoncelle")]
    Cello,
    #[serde(alias="Autre instru cordes")]
    OtherStringedInstruments,
    #[serde(alias="Autre instru vent")]
    OtherWoodwindInstruments,
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum CsvPublicationStatus {
    #[serde(alias="Archivé")]
    Archived,
    #[serde(alias="en cours", alias="inscription validée", alias="Validé")]
    Confirmed,
    #[serde(alias="")]
    ToBeChecked,
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum CsvArchivedStatus {
    #[serde(alias="True")]
    Archived,
    #[serde(alias="", alias="False")]
    Confirmed,
}


#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum CsvZapRole {
    #[serde(alias="recensement-3")]
    Recensement3,
    #[serde(alias="mbr-asso")]
    MbrAsso,
    #[serde(alias="mbr-asso-actif")]
    MbrAssoActif,
    #[serde(alias="mbr-asso-bienfaiteur")]
    MbrAssoBienfaiteur,
    #[serde(alias="référent")]
    Referent,
    #[serde(alias="leader")]
    Leader,
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum CsvProvenance {
    #[serde(alias="bassetorius")]
    Basetorius,
    #[serde(alias="easyzic")]
    Easyzic,
    #[serde(alias="flyers")]
    Flyers,
    #[serde(alias="autre")]
    Other,
    #[serde(alias="moteur de recherche")]
    SearchEngine,
    #[serde(alias="réseaux sociaux")]
    SocialNetwork,
    #[serde(alias="bouche à oreille")]
    WordsOfMouth,
    #[serde(alias="youtube")]
    Youtube,
    #[serde(alias="zikinf")]
    Zikinf,
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum CsvLocationType {
    #[serde(alias="Bar")]
    Cafe,
    #[serde(alias="Café Concert")]
    CafeConcert,
    #[serde(alias="Salle de concert")]
    ConcertHall,
    #[serde(alias="Studio de répétition")]
    RehearsalStudio,
    #[serde(alias="Salle de réunion")]
    MeetingHall,
    #[serde(alias="Restaurant")]
    Restaurant,
    #[serde(alias="Karaoké")]
    Karaoke,
    #[serde(alias="Lieu privé")]
    PrivateLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub enum CsvVolunteerType {
    ComputerScience,
    Designer,
    Illustrator,
    Photograph,
    Videographer,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub enum CsvCommunity {
    #[serde(alias="Avancés")]
    Interadv,
    #[serde(alias="Interdebs")]
    Interbeg,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub enum CsvEventType {
    #[serde(alias="Concert")]
    Concert,
    #[serde(alias="Jam")]
    JamSession,
    #[serde(alias="Scène ouverte")]
    OpenMic,
}

impl FromStr for CsvEncounterStatus {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Cancelled" => Ok(CsvEncounterStatus::Cancelled),
            "ClosedRegistration" => Ok(CsvEncounterStatus::ClosedRegistration),
            "OnDemandeRegistration" => Ok(CsvEncounterStatus::OnDemandeRegistration),
            "OpenRegistration" => Ok(CsvEncounterStatus::OpenRegistration),
            "Realised" => Ok(CsvEncounterStatus::Realised),
            _ => Err(ParseError(s.to_string())),
        }
    }
}

impl FromStr for CsvEncounterType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ok-jams-concerts" => Ok(CsvEncounterType::Concert),
            "ok-jamsdistancielles" => Ok(CsvEncounterType::DistantJam),
            "ok-jamostudios" => Ok(CsvEncounterType::RehearsalJam),
            "ok-sortiejams" => Ok(CsvEncounterType::OutJam),
            "ok-soireejams" => Ok(CsvEncounterType::EventJam),
            "ok-streetjams" => Ok(CsvEncounterType::StreetJam),
            "ok-karaokes" => Ok(CsvEncounterType::Karaoke),
            "ok-informelle" => Ok(CsvEncounterType::Informal),
            _ => Err(ParseError(s.to_string())),
        }
    }
}

impl FromStr for CsvEventStatus {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "à booker" => Ok(CsvEventStatus::ToBook),
            "booké" => Ok(CsvEventStatus::Booked),
            "confirmé et booké" => Ok(CsvEventStatus::BookedAndConfirmed),
            "annulé" => Ok(CsvEventStatus::Cancelled),
            "réalisé" => Ok(CsvEventStatus::Realised),
            _ => Err(ParseError(s.to_string())),
        }
    }
}


impl FromStr for CsvMusicGenre {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Blues" => Ok(CsvMusicGenre::Blues),
            "Chanson française" => Ok(CsvMusicGenre::FrenchSong),
            "Country" => Ok(CsvMusicGenre::Country),
            "Disco" => Ok(CsvMusicGenre::Disco),
            "Folk" => Ok(CsvMusicGenre::Folk),
            "Funk" => Ok(CsvMusicGenre::Funk),
            "Fusion" => Ok(CsvMusicGenre::Fusion),
            "Grunge" => Ok(CsvMusicGenre::Grunge),
            "Hard Rock" => Ok(CsvMusicGenre::HardRock),
            "Instrumental" => Ok(CsvMusicGenre::Instrumental),
            "Jazz" => Ok(CsvMusicGenre::Jazz),
            "Latin" => Ok(CsvMusicGenre::Latin),
            "Metal" => Ok(CsvMusicGenre::Metal),
            "Pop" => Ok(CsvMusicGenre::Pop),
            "Punk" => Ok(CsvMusicGenre::Punk),
            "RnB" => Ok(CsvMusicGenre::RnB),
            "Rock" => Ok(CsvMusicGenre::Rock),
            "Soul" => Ok(CsvMusicGenre::Soul),
            "World" => Ok(CsvMusicGenre::World),
            "Reggae" => Ok(CsvMusicGenre::Reggae),
            "Afrobeat" => Ok(CsvMusicGenre::Afrobeat),
            "Brésilien" => Ok(CsvMusicGenre::Brasilian),
            "Hip Hop" => Ok(CsvMusicGenre::HipHop),
            "Trad" => Ok(CsvMusicGenre::Trad),
            "De chambre" => Ok(CsvMusicGenre::Chamber),
            _ => Err(ParseError(s.to_string())),
        }
    }
}

impl FromStr for CsvParticipantType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Available" => Ok(CsvParticipantType::Available),
            "IfNeeded" => Ok(CsvParticipantType::IfNeeded),
            "NotAvailable" => Ok(CsvParticipantType::NotAvailable),
            _ => Err(ParseError(s.to_string())),
        }
    }
}

impl FromStr for CsvPlayedInstrument {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Tout Type d'Instrument" => Ok(CsvPlayedInstrument::All),
            "Accordéon" => Ok(CsvPlayedInstrument::Accordion),
            "Basse" => Ok(CsvPlayedInstrument::Bass),
            "Batterie" => Ok(CsvPlayedInstrument::Drums),
            "Cajon" => Ok(CsvPlayedInstrument::Cajon),
            "Chant" => Ok(CsvPlayedInstrument::Voice),
            "Clarinette" => Ok(CsvPlayedInstrument::Clarinet),
            "Clavier" => Ok(CsvPlayedInstrument::Keyboard),
            "Contrebasse" => Ok(CsvPlayedInstrument::DoubleBass),
            "Drum machine" => Ok(CsvPlayedInstrument::DrumMachine),
            "Flûte" => Ok(CsvPlayedInstrument::Flute),
            "Guitare" => Ok(CsvPlayedInstrument::Guitar),
            "Harmonica" => Ok(CsvPlayedInstrument::Harmonica),
            "MAO" => Ok(CsvPlayedInstrument::CAM),
            "Percussions" => Ok(CsvPlayedInstrument::Percussion),
            "Piano" => Ok(CsvPlayedInstrument::Piano),
            "Saxophone" => Ok(CsvPlayedInstrument::Saxophone),
            "Trombone" => Ok(CsvPlayedInstrument::Trombone),
            "Trompette" => Ok(CsvPlayedInstrument::Trumpet),
            "Violon" => Ok(CsvPlayedInstrument::Violin),
            "Harpe" => Ok(CsvPlayedInstrument::Harp),
            "+Violoncelle" => Ok(CsvPlayedInstrument::Cello),
            "Autre instru cordes" => Ok(CsvPlayedInstrument::OtherStringedInstruments),
            "Autre instru vent" => Ok(CsvPlayedInstrument::OtherWoodwindInstruments),
            _ => Err(ParseError(s.to_string())),
        }
    }
}

impl FromStr for CsvPublicationStatus {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Archivé" => Ok(CsvPublicationStatus::Archived),
            "Validé" => Ok(CsvPublicationStatus::Confirmed),
            "" => Ok(CsvPublicationStatus::ToBeChecked),
            _ => Err(ParseError(s.to_string())),
        }
    }
}

impl FromStr for CsvArchivedStatus {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "True" => Ok(CsvArchivedStatus::Archived),
            _ => Ok(CsvArchivedStatus::Confirmed),
        }
    }
}


impl FromStr for CsvZapRole {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mbr-asso" => Ok(CsvZapRole::MbrAsso),
            "mbr-asso-actif" => Ok(CsvZapRole::MbrAssoActif),
            "mbr-asso-bienfaiteur" => Ok(CsvZapRole::MbrAssoBienfaiteur),
            "recensement-3" => Ok(CsvZapRole::Recensement3),
            _ => Err(ParseError(s.to_string())),
        }
    }
}

impl FromStr for CsvProvenance {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bassetorius" => Ok(CsvProvenance::Basetorius),
            "easyzic" => Ok(CsvProvenance::Easyzic),
            "flyers" => Ok(CsvProvenance::Flyers),
            "autre" => Ok(CsvProvenance::Other),
            "moteur de recherche" => Ok(CsvProvenance::SearchEngine),
            "réseaux sociaux" => Ok(CsvProvenance::SocialNetwork),
            "bouche à oreille" => Ok(CsvProvenance::WordsOfMouth),
            "youtube" => Ok(CsvProvenance::Youtube),
            "zikinf" => Ok(CsvProvenance::Zikinf),
            _ => Err(ParseError(s.to_string())),
        }
    }
}


impl FromStr for CsvLocationType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Bar" => Ok(CsvLocationType::Cafe),
            "Café Concert" => Ok(CsvLocationType::CafeConcert),
            "Studio de répétition" => Ok(CsvLocationType::RehearsalStudio),
            "Salle de concert" => Ok(CsvLocationType::ConcertHall),
            "Salle de réunion" => Ok(CsvLocationType::MeetingHall),
            "Restaurant" => Ok(CsvLocationType::Restaurant),
            "Karaoké" => Ok(CsvLocationType::Karaoke),
            "Lieu privé" => Ok(CsvLocationType::PrivateLocation),
            _ => Err(ParseError(s.to_string())),
        }
    }
}

impl FromStr for CsvVolunteerType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ComputerScience" => Ok(CsvVolunteerType::ComputerScience),
            "Designer" => Ok(CsvVolunteerType::Designer),
            "Illustrator" => Ok(CsvVolunteerType::Illustrator),
            "Photograph" => Ok(CsvVolunteerType::Photograph),
            "Videographer" => Ok(CsvVolunteerType::Videographer),
            _ => Err(ParseError(s.to_string())),
        }
    }
}


impl FromStr for CsvCommunity {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Interdebs" => Ok(CsvCommunity::Interbeg),
            "Avancés" => Ok(CsvCommunity::Interadv),
            _ => Err(ParseError(s.to_string())),
        }
    }
}

impl FromStr for CsvEventType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Concert" => Ok(CsvEventType::Concert),
            "Scène ouverte" => Ok(CsvEventType::OpenMic),
            "Jam" => Ok(CsvEventType::JamSession),
            _ => Err(ParseError(s.to_string())),
        }
    }
}


#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid enum value: {}", self.0)
    }
}

impl std::error::Error for ParseError {}
