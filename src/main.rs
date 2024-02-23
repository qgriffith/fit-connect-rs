mod modules;
mod cli;

use crate::modules::strava::update_athlete_weight;
use crate::modules::withings::{get_day_before_timestamp, get_weight_by_date};
use std::process::exit;
use crate::cli::cli;

fn process_cli() -> i64 {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("withings", sub_matches)) => {
           let last = sub_matches.get_one::<i64>("LAST").expect("required");
           last.to_owned()
        }
        _ => unreachable!()
    }

}
fn get_and_format_weight(last: i64) -> Option<String> {
    match get_weight_by_date(get_day_before_timestamp(last)) {
        Ok(weight) => Some((weight / 1000.0).to_string()),
        Err(e) => {
            eprintln!("Failed to get weight for the last 24 hours {:?}", e);
            exit(1)
        }
    }
}

fn main() {
    let last = process_cli();
    if let Some(weight_in_kgs) = get_and_format_weight(last) {
        update_athlete_weight(&weight_in_kgs);
        println!("Weight updated in Strava to {} kg", weight_in_kgs)
    }
}
