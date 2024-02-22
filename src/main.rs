mod modules;

use crate::modules::strava::update_athlete_weight;
use crate::modules::withings::{get_day_before_timestamp, get_weight_by_date};
use std::process::exit;

fn get_and_format_weight() -> Option<String> {
    match get_weight_by_date(get_day_before_timestamp()) {
        Ok(weight) => Some((weight / 1000.0).to_string()),
        Err(e) => {
            eprintln!("Failed to get weight for the last 24 hours {:?}", e);
            exit(1)
        }
    }
}

fn main() {
    if let Some(weight_in_kgs) = get_and_format_weight() {
        update_athlete_weight(&weight_in_kgs);
        println!("Weight updated in Strava to {} kg", weight_in_kgs)
    }
}
