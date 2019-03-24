extern crate clap;

extern crate apex_stats;
use apex_stats::backend::Type;
use apex_stats::{make_backend, make_config, make_console_interface, Interface};

fn main() {
    let config = make_config().expect("Could not create config");
    config
        .ensure_data_path_exists()
        .expect("Could not create data directories");
    let mut console = make_console_interface();
    let mut backend = make_backend(Type::CSVLog, &config).expect("Fix this");

    match console.gather_observation() {
        Ok(observation) => backend.record(observation).expect("This should succeed"),
        Err(err) => panic!("Something went wrong"),
    };
}
