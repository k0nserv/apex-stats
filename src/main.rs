use std::error::Error;
extern crate clap;

extern crate apex_stats;
use apex_stats::backend::Type;
use apex_stats::{make_backend, make_config, make_console_interface};

fn main() -> Result<(), Box<dyn Error>> {
    let config = make_config().expect("Could not create config");
    config.ensure_data_path_exists()?;

    let mut console = make_console_interface();
    let mut backend = make_backend(Type::CSVLog, &config).expect("Fix this");

    let observation = console.gather_observation()?;
    backend.record(&observation)?;

    if observation.is_win() {
        console.message("Congrats on the win\n");
    } else if observation.is_top_three() {
        console.message("Top three, not too shabby\n");
    } else if observation.is_last() {
        console.message("Better luck next time\n");
    }

    Ok(())
}
