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
        state.serialize_field("number_of_kills", &self.number_of_kills)?;
        state.serialize_field("damage_dealt", &self.damage_dealt)?;
        state.serialize_field("squad_position", &self.squad_position)?;
        state.serialize_field("legend", &self.legend)?;
        state.serialize_field("at", &self.at)?;
        state.serialize_field("squad_type", &self.squad_type)?;
        state.serialize_field("notes", &self.notes)?;
        state.end()
    }
}
