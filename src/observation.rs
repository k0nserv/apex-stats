use chrono::{DateTime, Local};

use crate::apex::{Legend, SquadType};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Observation {
    #[serde(rename = "Kills")]
    pub number_of_kills: u64,

    #[serde(rename = "Damage")]
    pub damage_dealt: u64,

    #[serde(rename = "Squad Position")]
    pub squad_position: u64,

    #[serde(rename = "Legend")]
    pub legend: Legend,

    #[serde(rename = "Time")]
    pub at: DateTime<Local>,

    #[serde(rename = "Squad Makeup")]
    pub squad_type: SquadType,

    #[serde(rename = "Notes")]
    pub notes: String,
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
