use std::error::Error;
use std::process::Command;

#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};

use chrono::{DateTime, Duration, Local};

extern crate apex_stats;
use apex_stats::backend::Type;
use apex_stats::stats::{Query, QueryResult};
use apex_stats::{
    make_backend, make_config, make_console_interface, Backend, Interface, Legend, SquadType,
};

const ADD_SUBCOMMAND_NAME: &'static str = "add";
const SHOW_SUBCOMMAND_NAME: &'static str = "show";
const OPEN_LOG_LOCATION_SUBCOMMAND_NAME: &'static str = "open";

fn record_match(interface: &mut Interface, backend: &mut Backend) -> Result<(), Box<dyn Error>> {
    let observation = interface.gather_observation()?;
    backend.record(&observation)?;

    if observation.is_win() {
        interface.message("Congrats on the win\n");
    } else if observation.is_top_three() {
        interface.message("Top three, not too shabby\n");
    } else if observation.is_last() {
        interface.message("Better luck next time\n");
    }

    Ok(())
}

fn print_stats(
    interface: &mut Interface,
    backend: &Backend,
    matches: &ArgMatches,
) -> Result<(), Box<dyn Error>> {
    // TODO: These should not fail silently
    let legend: Option<Legend> = matches
        .value_of("legend")
        .and_then(|value| value.trim().parse().ok());

    let squad_type: Option<SquadType> = matches
        .value_of("squad-makeup")
        .and_then(|value| value.trim().parse().ok());

    let after: Option<DateTime<Local>> = matches
        .value_of("after")
        .and_then(|value| value.trim().parse().ok());

    let week_filter = if matches.is_present("last-week") {
        let now = Local::now();
        Some(now - Duration::weeks(1))
    } else {
        None
    };

    let mut query = Query::new();
    if let Some(legend) = legend {
        query = query.match_legend(legend);
    }

    if let Some(squad_type) = squad_type {
        query = query.match_squad_type(squad_type)
    }

    if let Some(after) = after {
        query = query.after(after)
    }

    if let Some(week_filter) = week_filter {
        query = query.after(week_filter)
    };

    if let Some(result) = query.execute(backend.all_records()?) {
        interface.message(&format!("Max Damage: {}\n", result.max_damage));
        interface.message(&format!("Max Kills: {}\n", result.max_kills));
        interface.message(&format!("ADR: {:.2}\n", result.average_damager_per_round()));
        interface.message(&format!("AKR: {:.2}\n", result.average_kills_per_round()));
        interface.message(&format!("Number of games: {}\n", result.number_of_matches));
        interface.message(&format!("Total Damage: {}\n", result.total_damage));
        interface.message(&format!("Total Kills: {}\n", result.total_kills));
        Ok(())
    } else {
        interface.message("No observations recorded.");
        return Ok(());
    }
}

fn open_log_location(data_dir: &str) -> Result<(), Box<dyn Error>> {
    if cfg!(target_os = "windows") {
        Command::new("explorer").args(&[data_dir]).output()?;
    } else if cfg!(target_os = "macos") {
        Command::new("open").args(&[data_dir]).output()?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("apex-stats")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Record your Apex Legends stats for analysis and other nerdiness")
        .subcommand(SubCommand::with_name(ADD_SUBCOMMAND_NAME))
        .subcommand(
            SubCommand::with_name(SHOW_SUBCOMMAND_NAME)
                .about("Show a summary of the collected stats. This summary can be filtered")
                .args(&[
                    Arg::with_name("legend")
                        .help("Show only the stats for a given legend")
                        .takes_value(true)
                        .long("legend")
                        .short("l"),
                    Arg::with_name("squad-makeup")
                        .help("Show only the stats for a given squad makeup")
                        .takes_value(true)
                        .long("squad-makeup")
                        .short("s"),
                    Arg::with_name("after")
                        .help("Only show observations added after a specific point in time")
                        .takes_value(true)
                        .long("after"),
                    Arg::with_name("last-week")
                        .help("Only show observations in the last week")
                        .long("last-week"),
                ]),
        )
        .subcommand(
            SubCommand::with_name(OPEN_LOG_LOCATION_SUBCOMMAND_NAME)
                .about("Open the location of the log file in a window"),
        )
        .get_matches();

    let config = make_config().expect("Could not create config");
    config.ensure_data_path_exists()?;

    let mut console = make_console_interface();
    let mut backend = make_backend(Type::CSVLog, &config).expect("Fix this");
    match matches.subcommand() {
        ("", None) | (ADD_SUBCOMMAND_NAME, Some(_)) => {
            record_match(console.as_mut(), backend.as_mut())?;

            Ok(())
        }
        (SHOW_SUBCOMMAND_NAME, Some(print_matches)) => {
            print_stats(console.as_mut(), backend.as_ref(), print_matches)
        }
        (OPEN_LOG_LOCATION_SUBCOMMAND_NAME, _) => open_log_location(
            config
                .data_dir()
                .to_str()
                .expect("Unwrapping a well-formed path should never fail"),
        ),
        _ => Ok(()),
    }
}
