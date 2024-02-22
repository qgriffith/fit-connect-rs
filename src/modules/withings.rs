use chrono::{DateTime, Duration, Local};
use std::env;
use std::path::Path;
use std::process::exit;
use withings_rs::{
    api::{auth, measure},
    models::{meas::CategoryType, MeasureType},
};
fn get_env_var(name: &str) -> String {
    env::var(name).unwrap_or_else(|e| {
        eprint!("{} env var not set", name);
        exit(1);
    })
}

fn get_access_token() -> Result<String, String> {
    let client_secret = get_env_var("WITHINGS_CLIENT_SECRET");
    let client_id = get_env_var("WITHINGS_CLIENT_ID");
    let config_file = auth::get_config_file();

    let access_token = if Path::new(&config_file).exists() {
        auth::refresh_token(client_id, client_secret)
    } else {
        auth::get_access_code(client_id, client_secret)
    };

    Ok(access_token.unwrap().to_string())
}

pub fn get_weight_by_date(lastupdate: String) -> Result<f64, String> {
    let access_token_res = get_access_token();
    if access_token_res.is_err() {
        return Err(access_token_res.err().unwrap());
    }
    let category = CategoryType::Measures.to_string();
    let weight = MeasureType::Weight.to_string();

    let access_token = access_token_res.unwrap();
    let params = measure::MeasurementParams {
        access_token,
        client_id: get_env_var("WITHINGS_CLIENT_ID"),
        category,
        meastype: weight,
        start: None,
        end: None,
        offset: None,
        lastupdate: Some(lastupdate.to_string()),
    };
    let measurements_res = measure::get_measurements(&params);
    if measurements_res.is_err() {
        return Err(measurements_res.err().unwrap().to_string());
    }

    let measurements = measurements_res.unwrap();
    if measurements.body.measuregrps.is_empty()
        || measurements.body.measuregrps[0].measures.is_empty()
    {
        return Err("No measurements received".to_string());
    }

    Ok(measurements.body.measuregrps[0].measures[0].value as f64)
}

/// Returns the timestamp of the day before the current time.
/// Get the last weight for 24 hours. Withings lastupdate needs to be
/// set to epoch time of 24hours prior to the days weight you want to get
/// # Returns
///
/// - A string containing the timestamp of the day before the current time.
pub fn get_day_before_timestamp() -> String {
    let current_time: DateTime<Local> = Local::now();
    let a_day_before = current_time - Duration::days(1);
    let day_before_timestamp = a_day_before.timestamp();

    day_before_timestamp.to_string()
}
