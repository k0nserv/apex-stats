use chrono::{DateTime, Local};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::apex::{Legend, SquadType};

#[derive(Debug, PartialEq, Eq)]
pub struct Observation {
    pub number_of_kills: u64,
    pub damage_dealt: u64,
    pub squad_position: u64,
    pub legend: Legend,
    pub squad_type: SquadType,
    pub notes: String,
    pub at: DateTime<Local>,
}

impl Serialize for Observation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Observation", 7)?;
        state.serialize_field("Kills", &self.number_of_kills)?;
        state.serialize_field("Damage", &self.damage_dealt)?;
        state.serialize_field("Squad Position", &self.squad_position)?;
        state.serialize_field("Legend", &self.legend)?;
        state.serialize_field("Time", &self.at)?;
        state.serialize_field("Sqaud Makeup", &self.squad_type)?;
        state.serialize_field("Notes", &self.notes)?;
        state.end()
    }
}

impl Observation {
    pub fn is_win(&self) -> bool {
        self.squad_position == 1
    }

    pub fn is_top_three(&self) -> bool {
        self.squad_position < 4
    }

    pub fn is_last(&self) -> bool {
        self.squad_position >= 20
    }
}
