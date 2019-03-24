use chrono::{DateTime, Local};
use std::io::prelude::*;
use std::str::FromStr;

use crate::{Legend, Observation, SquadType};

use super::Interface;

pub trait LineReader {
    fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize>;
}

pub struct Console<'a> {
    reader: Box<dyn LineReader>,
    write: Box<dyn Write>,
    now: &'a Fn() -> DateTime<Local>,
}

impl<'a> Console<'a> {
    pub fn new<F>(reader: Box<dyn LineReader>, write: Box<dyn Write>, now: &'a F) -> Self
    where
        F: Fn() -> DateTime<Local>,
    {
        Self { reader, write, now }
    }

    fn read_input<T: FromStr>(&mut self, message: &str) -> std::io::Result<T> {
        self.message(message);

        loop {
            let mut buffer = String::new();
            self.reader.read_line(&mut buffer)?;

            match buffer.trim().parse::<T>() {
                Ok(value) => return Ok(value),
                Err(_) => {
                    self.message("Invalid value. Please try again\n");
                    self.message("> ");
                }
            };
        }
    }
}

impl<'a> Interface for Console<'a> {
    fn gather_observation(&mut self) -> Result<Observation, Box<dyn std::error::Error>> {
        let number_of_kills = self.read_input::<u64>("Number of kills: ")?;
        let damage_dealt = self.read_input::<u64>("Damage dealt: ")?;
        let squad_position = self.read_input::<u64>("Squad Position: ")?;
        let legend = self.read_input::<Legend>("Legend: ")?;
        let squad_type = self.read_input::<SquadType>("Squad makeup: ")?;
        let notes = self.read_input::<String>("Notes: ")?;

        Ok(Observation {
            number_of_kills,
            damage_dealt,
            squad_position: squad_position,
            legend: legend,
            squad_type: squad_type,
            notes: notes,
            at: (self.now)(),
        })
    }

    fn message(&mut self, message: &str) {
        write!(self.write, "{}", message).expect("Failed to write message");
        self.write.flush().expect("Failed to flush output");
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::io::prelude::*;
    use std::rc::Rc;
    use std::time::SystemTime;

    use super::test_support::{make_test_console, WriteStorage};
    use super::{Console, Interface, Legend, Observation, SquadType};

    #[test]
    fn test_message() {
        // Arrange
        let (mut console, storage) = make_test_console(None, &|| SystemTime::UNIX_EPOCH);

        // Act
        console.message("Hello World");
        console.message("Hello World again");

        // Assert
        assert_eq!(
            vec![b"Hello World".to_vec(), b"Hello World again".to_vec()],
            storage.borrow().written
        );
    }

    #[test]
    fn test_gather_observation() {
        // Arrange
        let (mut console, storage) = make_test_console(
            Some(vec![
                "6".to_string(),                             // Number of kills
                "1142".to_string(),                          // Damage dealt
                "1".to_string(),                             // Squad position
                "Wraith".to_string(),                        // Legend
                "trio".to_string(),                          // Squad Type
                "Last guy died outside ring :D".to_string(), // Notes
            ]),
            &|| SystemTime::UNIX_EPOCH,
        );

        // Act
        let observation = console.gather_observation();

        // Assert
        let expected_observation = Observation {
            number_of_kills: 6,
            damage_dealt: 1142,
            squad_position: 1,
            legend: Legend::Wraith,
            squad_type: SquadType::Trio,
            notes: String::from("Last guy died outside ring :D"),
            at: SystemTime::UNIX_EPOCH,
        };

        assert_eq!(
            observation.expect("Shouldn't have failed"),
            expected_observation
        );
    }

    #[test]
    fn test_gather_observation_with_invalid_data() {
        // Arrange
        let (mut console, storage) = make_test_console(
            Some(vec![
                "Clearly to_string a number".to_string(), // Number of kills first attempt
                "6".to_string(),                          // Number of kills
                "Again to_string a number".to_string(),
                "1142".to_string(),   // Damage dealt
                "1".to_string(),      // Squad position
                "Stve".to_string(),   // Not a legend
                "Wraith".to_string(), // Legend
                "quartet".to_string(),
                "trio".to_string(),                          // Squad Type
                "Last guy died outside ring :D".to_string(), // Notes
            ]),
            &|| SystemTime::UNIX_EPOCH,
        );

        // Act
        let observation = console.gather_observation();

        // Assert
        let expected_observation = Observation {
            number_of_kills: 6,
            damage_dealt: 1142,
            squad_position: 1,
            legend: Legend::Wraith,
            squad_type: SquadType::Trio,
            notes: String::from("Last guy died outside ring :D"),
            at: SystemTime::UNIX_EPOCH,
        };

        assert_eq!(
            observation.expect("Shouldn't have failed"),
            expected_observation
        );
    }
}

#[cfg(test)]
mod test_support {
    use std::cell::RefCell;
    use std::io::prelude::*;
    use std::rc::Rc;
    use std::time::SystemTime;

    use super::{Console, LineReader};

    pub struct WriteStorage {
        pub written: Vec<Vec<u8>>,
    }

    struct MockWriter {
        pub storage: Rc<RefCell<WriteStorage>>,
    }

    impl MockWriter {
        fn new() -> Self {
            Self {
                storage: Rc::new(RefCell::new(WriteStorage { written: vec![] })),
            }
        }
    }

    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.storage.borrow_mut().written.push(buf.to_vec());

            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockReader {
        inputs: Vec<String>,
        current_input_index: usize,
    }

    impl MockReader {
        fn new(inputs: Vec<String>) -> Self {
            Self {
                inputs,
                current_input_index: 0,
            }
        }
    }

    impl LineReader for MockReader {
        fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
            let current_input = &self.inputs[self.current_input_index];
            buf.insert_str(0, current_input);
            self.current_input_index += 1;

            Ok(current_input.len())
        }
    }

    pub fn make_test_console<F>(
        reader_inputs: Option<Vec<String>>,
        now: &F,
    ) -> (Console, Rc<RefCell<WriteStorage>>)
    where
        F: Fn() -> SystemTime,
    {
        let mock_writer = MockWriter::new();
        let mock_reader = MockReader::new(reader_inputs.unwrap_or(vec![]));
        let storage = Rc::clone(&mock_writer.storage);

        (
            Console::new(Box::new(mock_reader), Box::new(mock_writer), now),
            storage,
        )
    }
}
