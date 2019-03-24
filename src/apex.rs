use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Serialize, Serializer};

#[derive(Debug, Serialize, PartialEq, Eq)]
pub enum Legend {
    Bangalore,
    Bloodhound,
    Caustic,
    Gibraltar,
    Lifeline,
    Mirage,
    Octane,
    Pathfinder,
    Wraith,
}

#[derive(Debug)]
pub struct LegendParseError {
    legend_name: String,
}

impl LegendParseError {
    fn new(legend_name: String) -> Self {
        Self { legend_name }
    }
}

impl Display for LegendParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Unknown legend {}", self.legend_name)
    }
}

impl Error for LegendParseError {}

impl FromStr for Legend {
    type Err = LegendParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "bangalore" => Ok(Legend::Bangalore),
            "bloodhound" => Ok(Legend::Bloodhound),
            "caustic" => Ok(Legend::Caustic),
            "gibraltar" => Ok(Legend::Gibraltar),
            "lifeline" => Ok(Legend::Lifeline),
            "mirage" => Ok(Legend::Mirage),
            "octane" => Ok(Legend::Octane),
            "pathfinder" => Ok(Legend::Pathfinder),
            "wraith" => Ok(Legend::Wraith),
            _ => Err(LegendParseError::new(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct SquadTypeParseError {
    squad_type: String,
}

impl SquadTypeParseError {
    fn new(squad_type: String) -> Self {
        Self { squad_type }
    }
}

impl Display for SquadTypeParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            r#"Unknown squad type "{}". Supported types are `solo`, `duo` and `trio`."#,
            self.squad_type
        )
    }
}

impl Error for SquadTypeParseError {}

#[derive(Debug, PartialEq, Eq)]
pub enum SquadType {
    Solo,
    Duo,
    Trio,
    Unknown,
}

impl FromStr for SquadType {
    type Err = SquadTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "solo" => Ok(SquadType::Solo),
            "duo" => Ok(SquadType::Duo),
            "trio" => Ok(SquadType::Trio),
            _ => Err(SquadTypeParseError::new(s.to_owned())),
        }
    }
}

impl Serialize for SquadType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SquadType::Solo => serializer.serialize_str("solo"),
            SquadType::Duo => serializer.serialize_str("duo"),
            SquadType::Trio => serializer.serialize_str("trio"),
            SquadType::Unknown => serializer.serialize_str("unknown"),
        }
    }
}
