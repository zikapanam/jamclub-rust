use std::fmt;
use super::sea_orm_active_enums::{Community, EncounterType};

impl fmt::Display for EncounterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format the struct as a string and write it to the formatter
        match self {
            EncounterType::Concert => write!(f, "Concert"),
            EncounterType::DistantJam => write!(f, "Jams distancielle"),
            EncounterType::RehearsalJam => write!(f, "Jam-O-Studio"),
            EncounterType::OutJam => write!(f, "Sortie jam"),
            EncounterType::EventJam => write!(f, "Soirée jam"),
            EncounterType::StreetJam => write!(f, "Street jam"),
            EncounterType::Karaoke => write!(f, "Karaoké"),
            EncounterType::Informal => write!(f, "Réunion informelle"),            
        }
    }
}


impl fmt::Display for Community {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format the struct as a string and write it to the formatter
        match self {
            Community::Interbeg => write!(f, "Interdebs"),
            Community::Interadv => write!(f, "Avancés"),
        }
    }
}

pub trait DisplayVec {
    fn display_vec(&self, f: &mut fmt::Formatter) -> fmt::Result;
}


impl<T> DisplayVec for Vec<T>
where
    T: fmt::Display, // T must implement fmt::Display
{
    fn display_vec(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format the struct as a string and write it to the formatter
        let mut first: bool = true;
        for e in self {
            if !first {
                write!(f, ", ");
            }
            write!(f, "{}", e);
            first = false;
        }
        Ok(())
    }
}

pub struct DisplayableVec<T>(pub Vec<T>);

impl<T> fmt::Display for DisplayableVec<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.display_vec(f)
    }
}
