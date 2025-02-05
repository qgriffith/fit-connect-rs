use crate::modules::withings::{get_day_before_timestamp, get_weight_by_date};
use std::process::exit;

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
            eprintln!("Failed to get weight for the polling period {:?}", e);
            exit(1)
        }
    }
}
