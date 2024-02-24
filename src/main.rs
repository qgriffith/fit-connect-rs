mod modules;
mod cli;

use crate::modules::withings::{get_day_before_timestamp, get_weight_by_date};
use std::process::exit;
use crate::cli::cli;
use crate::modules::strava::update_athlete_weight;

fn process_cli()  {
    match cli().subcommand() {
        Some(("withings", withing_matches )) => {
            if withing_matches.contains_id("last") {
                let last = withing_matches.get_one::<i64>("last").expect("contains_id");
                println!("Getting weight for the day {}\n", last);
                let weight_in_kgs = get_and_format_weight(last.to_owned());

                if withing_matches.get_flag("strava-sync") {
                    let strava_sync = withing_matches.get_one::<bool>("strava-sync").expect("is present");
                    println!("Syncing to Strava...\n");
                    let weight = weight_in_kgs.unwrap();
                    update_athlete_weight(weight.as_str());
                    println!("Weight updated in Strava to {} kg\n", weight)
                }
            }
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
}
