use std::io;

use chrono::Local;

use crate::observation::Observation;
mod console;
pub use console::{Console, LineReader};

pub trait Interface {
    fn gather_observation(&mut self) -> Result<Observation, Box<dyn std::error::Error>>;
    fn message(&mut self, message: &str);
}

struct StdinLineReader {
    stdin: io::Stdin,
}

impl StdinLineReader {
    fn new(stdin: io::Stdin) -> Self {
        Self { stdin }
    }
}

impl LineReader for StdinLineReader {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        self.stdin.read_line(buf)
    }
}

pub fn make_console_interface() -> Box<dyn Interface> {
    Box::new(Console::new(
        Box::new(StdinLineReader::new(io::stdin())),
        Box::new(io::stdout()),
        &|| Local::now(),
    ))
}
