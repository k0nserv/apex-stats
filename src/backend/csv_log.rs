use std::fs::{metadata, OpenOptions};
use std::io;
use std::path::PathBuf;

use super::{Backend, Result};

use crate::observation::Observation;

enum CSVLogError {
    IOError(io::Error),
}

impl From<io::Error> for CSVLogError {
    fn from(io_error: io::Error) -> CSVLogError {
        CSVLogError::IOError(io_error)
    }
}

pub struct CSVLog {
    path: PathBuf,
}

impl CSVLog {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn record_file_exists(&self) -> bool {
        metadata(&self.path).is_ok()
    }
}

impl Backend for CSVLog {
    fn record(&mut self, observation: Observation) -> Result<()> {
        let record_file_exists = self.record_file_exists();

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        let mut writer = csv::WriterBuilder::new()
            .has_headers(!record_file_exists)
            .from_writer(file);

        writer.serialize(observation)?;
        writer.flush()?;

        Ok(())
    }
}
