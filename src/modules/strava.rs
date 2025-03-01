use std::env;
use std::path::Path;
use strava_client_rs::api::{athlete, auth};
use strava_client_rs::models::athlete::AthleteStats;
use strava_client_rs::models::AthleteCollection;
use strava_client_rs::util::auth_config;
const AUTH_URL: &'static str = "http://www.strava.com/oauth/authorize";
const TOKEN_URL: &'static str = "https://www.strava.com/oauth/token";
pub fn get_authed_athlete() -> AthleteCollection {
    let access_token = create_and_get_access_token();
    let athlete = athlete::get_athlete(access_token.as_str()).unwrap();
    athlete
}

pub fn update_athlete_weight(weight: &str) -> String {
    let access_token = create_and_get_access_token();
    let athlete = athlete::update_athlete_weight(access_token.as_str(), weight).unwrap();
    athlete.status().to_string()
}

pub fn get_athlete_stats() -> Result<AthleteStats, Box<dyn std::error::Error>> {
    let access_token = create_and_get_access_token();

    let auth_athlete = get_authed_athlete();
    let athlete_stats =
        athlete::get_athlete_stats(access_token.as_str(), &auth_athlete.id.to_string()).unwrap();
    Ok(athlete_stats)
}

fn create_and_get_access_token() -> String {
    let config_file = env::var("STRAVA_CONFIG_FILE").unwrap_or_else(|_| "config.json".to_string());
    get_access_token(config_file).unwrap()
}
fn get_access_token(config_file: String) -> Result<String, String> {
    let client_id =
        env::var("STRAVA_CLIENT_ID").expect("Missing the STRAVA_CLIENT_ID environment variable.");
    let client_secret = env::var("STRAVA_CLIENT_SECRET")
        .expect("Missing the STRAVA_CLIENT_SECRET environment variable.");

    // Setup default config for auth
    let mut config = auth::Config::new(
        client_id.to_string(),
        client_secret.to_string(),
        Default::default(), // no refresh token so set to default which is none
        AUTH_URL.to_string(),
        TOKEN_URL.to_string(),
    );

    // Check if the config file exists and get the access token or get a new one
    if Path::new(&config_file).exists() {
        config.refresh_token = Some(auth_config::config_file::load_config().refresh_token);
        let refresh_access_token = auth::get_refresh_token(config);
        Ok(refresh_access_token.unwrap().to_string())
    } else {
        let access_token = auth::get_authorization(config);
        Ok(access_token.unwrap().to_string())
    }
}

/// Synchronizes the athlete's weight to Strava.
///
/// # Arguments
///
/// * `weight_in_kgs` - The athlete's weight in kilograms. If `None`, the function panics.
///
/// # Panics
///
/// The function panics if `weight_in_kgs` is `None`.
pub fn sync_strava(weight_in_kgs: Option<String>) {
    println!("Syncing to Strava...\n");
    let weight = weight_in_kgs.unwrap();
    update_athlete_weight(weight.as_str());
    println!("Weight updated in Strava to {} kg\n", weight);
}
