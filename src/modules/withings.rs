//! Withings API integration module for retrieving weight measurements
//!
//! This module provides functionality to authenticate with the Withings API
//! and retrieve weight measurements for specified dates.

use chrono::{DateTime, Duration, Local};
use miette::{Context, IntoDiagnostic, Result};
use std::{env, path::Path};
use withings_rs::{
    api,
    api::{auth, measure},
    models::{meas::CategoryType, MeasureType},
};

/// Errors that can occur during Withings API operations
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum WithingsError {
    /// Represents configuration-related errors
    #[error("Configuration error: {message}")]
    #[diagnostic(code(withings::config::invalid))]
    Config {
        /// Description of the configuration error
        message: String,
        /// Helpful suggestion to resolve the error
        #[help]
        help: String,
    },
}

/// Authentication configuration for Withings API
const AUTH_CONFIG: WithngsAuthConfig = WithngsAuthConfig {
    client_id_env: "WITHINGS_CLIENT_ID",
    client_secret_env: "WITHINGS_CLIENT_SECRET",
};
/// Structure holding environment variable names for authentication
struct WithngsAuthConfig {
    /// Environment variable name for client ID
    client_id_env: &'static str,
    /// Environment variable name for client secret
    client_secret_env: &'static str,
}

/// Errors that can occur during weight measurement operations
#[derive(thiserror::Error, Debug)]
pub enum WeightError {
    /// Authentication-related errors
    #[error("Authentication error: {0}")]
    Auth(String),
    /// Measurement retrieval errors
    #[error("Measurement error: {0}")]
    Measurement(String),
    /// No measurements found for the requested period
    #[error("No measurements available")]
    NoMeasurements,
}

/// Retrieves an environment variable value
///
/// # Arguments
///
/// * `name` - Name of the environment variable to retrieve
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `String` - The value of the environment variable
/// * `WithingsError` - Error if the variable is not set
///
/// # Examples
///
/// ```rust
/// let api_key = get_env_var("API_KEY")?;
/// ```
fn get_env_var(name: &str) -> Result<String> {
    env::var(name)
        .map_err(|_| WithingsError::Config {
            message: format!("Missing environment variable {}", name),
            help: format!("Set the {} environment variable", name),
        })
        .into_diagnostic()
}

/// Retrieves or refreshes the Withings API access token
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `String` - The access token
/// * `WithingsError` - Error if token retrieval fails
///
/// # Examples
///
/// ```rust
/// let token = get_access_token()?;
/// ```
fn get_access_token() -> Result<String> {
    let client_secret =
        get_env_var(AUTH_CONFIG.client_secret_env).wrap_err("Missing client secret")?;
    let client_id = get_env_var(AUTH_CONFIG.client_id_env).wrap_err("Missing client ID")?;

    let config_file = api::config::get_config_file();

    let access_token = if Path::new(&config_file).exists() {
        auth::refresh_token(client_id, client_secret)
    } else {
        auth::get_access_code(client_id, client_secret)
    };

    access_token
        .map(|token| token.to_string())
        .map_err(|e| WithingsError::Config {
            message: "Failed to obtain access token".to_string(),
            help: format!("Error: {}", e),
        })
        .into_diagnostic()
}

/// Retrieves weight measurement for a specific date from Withings API
///
/// # Arguments
///
/// * `lastupdate` - Timestamp string representing the date after which to fetch measurements
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `f64` - The weight measurement in grams
/// * `WeightError` - Error that occurred during retrieval
///
/// # Errors
///
/// This function will return an error if:
/// * Authentication fails
/// * API request fails
/// * No measurements are available
///
/// # Examples
///
/// ```rust
/// let weight = get_weight_by_date("1634567890")?;
/// println!("Weight: {}g", weight);
/// ```
pub fn get_weight_by_date(lastupdate: String) -> Result<f64, WeightError> {
    // Get authentication tokens
    let access_token = get_access_token().map_err(|e| WeightError::Auth(e.to_string()))?;
    let client_id =
        get_env_var(AUTH_CONFIG.client_id_env).map_err(|e| WeightError::Auth(e.to_string()))?;

    // Prepare measurement parameters
    let params = measure::MeasurementParams {
        access_token,
        client_id,
        category: CategoryType::Measures.to_string(),
        meastype: MeasureType::Weight.to_string(),
        start: None,
        end: None,
        offset: None,
        lastupdate: Some(lastupdate.to_string()),
    };

    // Get measurements
    let measurements =
        measure::get_measurements(&params).map_err(|e| WeightError::Measurement(e.to_string()))?;

    // Extract first measurement or return error if none exists
    let measuregrp = measurements
        .body
        .measuregrps
        .first()
        .ok_or(WeightError::NoMeasurements)?;
    let measure = measuregrp
        .measures
        .first()
        .ok_or(WeightError::NoMeasurements)?;

    Ok(measure.value as f64)
}

/// Calculates a timestamp for a specified number of days before the current date
///
/// # Arguments
///
/// * `day` - Number of days to subtract from the current date
///
/// # Returns
///
/// Returns a string containing the Unix timestamp for the calculated date
///
/// # Examples
///
/// ```rust
/// // Get timestamp for yesterday
/// let yesterday = get_day_before_timestamp(1);
/// ```
pub fn get_day_before_timestamp(day: i64) -> String {
    let current_time: DateTime<Local> = Local::now();
    let a_day_before = current_time - Duration::days(day);
    let day_before_timestamp = a_day_before.timestamp();

    day_before_timestamp.to_string()
}
