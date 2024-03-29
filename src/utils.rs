use std::process::exit;
use crate::cli::cli;
use crate::modules::strava;
use crate::modules::withings::{get_day_before_timestamp, get_weight_by_date};

/// Retrieves the weight from the previous day and formats it as a string.
///
/// # Arguments
///
/// * `day_offset` - The days to get the weight from 1 == current day, 2 == previous
///
/// # Returns
///
/// An `Option<String>` representing the weight from the previous day, converted to kilograms.
/// Returns `None` if an error occurs during retrieval of the weight or formatting.
pub fn get_and_format_weight(day_offset: i64) -> Option<String> {
    match get_weight_by_date(get_day_before_timestamp(day_offset)) {
        Ok(weight) => Some((weight / 1000.0).to_string()),
        Err(e) => {
            eprintln!("Failed to get weight for the last 24 hours {:?}", e);
            exit(1)
        }
    }
}

/// Process the command-line interface (CLI)
///
/// This function processes the CLI subcommand and performs the required actions based on the subcommand.
pub fn process_cli() {
    match cli().subcommand() {
        Some(("withings", withing_matches)) => {
            if withing_matches.contains_id("last") {
                let day_offset = withing_matches.get_one::<i64>("last").expect("contains_id");
                println!("Getting weight for the day from {}\n", day_offset);
                let weight_in_kgs = get_and_format_weight(day_offset.to_owned());
                if withing_matches.get_flag("strava-sync") {
                    strava::sync_strava(weight_in_kgs);
                }
            }
        }
        _ => unreachable!(),
    }
}
