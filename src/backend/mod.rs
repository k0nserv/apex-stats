mod csv_log;

use std::error::Error;
use std::result;

use crate::config::Config;
use crate::observation::Observation;
use csv_log::CSVLog;

pub enum Type {
    CSVLog,
}

pub type Result<T> = result::Result<T, Box<Error>>;

pub trait Backend {
    fn record(&mut self, observation: Observation) -> Result<()>;
}

pub fn make_backend(kind: Type, config: &Config) -> Result<Box<Backend>> {
    match kind {
        Type::CSVLog => Ok(Box::new(CSVLog::new(config.data_path()))),
    }
}
