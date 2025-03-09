//! Strava API client module for athlete management and authentication.
//!
//! This module provides functionality to interact with the Strava API,
//! including authentication, athlete data retrieval, and weight updates.

use miette::{Context, IntoDiagnostic, Result};
use std::{env, path::Path};

use strava_client_rs::api::{athlete, auth};
use strava_client_rs::models::{AthleteCollection, AthleteStats};
use strava_client_rs::util::auth_config;

/// Possible errors that can occur during Strava API operations.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum StravaError {
    /// Authentication-related errors, such as invalid tokens or failed authentication attempts.
    #[error("Authentication failed")]
    #[diagnostic(code(strava::auth::failed))]
    Authentication {
        /// The underlying error that caused the authentication failure
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
        /// Helpful message for resolving the authentication issue
        #[help]
        help: Option<String>,
    },

    /// Configuration-related errors, such as missing environment variables.
    #[error("Configuration error: {message}")]
    #[diagnostic(code(strava::config::invalid))]
    Config {
        /// Description of the configuration error
        message: String,
        /// Guidance on how to fix the configuration
        #[help]
        help: String,
    },

    /// API-related errors, such as failed requests or invalid responses.
    #[error("API error: {message}")]
    #[diagnostic(code(strava::api::error))]
    Api {
        /// Description of the API error
        message: String,
        /// Additional context about the API call
        #[source_code]
        src: Option<String>,
    },
}

/// Authentication configuration for Strava API.
const AUTH_CONFIG: StravaAuthConfig = StravaAuthConfig {
    auth_url: "http://www.strava.com/oauth/authorize",
    token_url: "https://www.strava.com/oauth/token",
    config_file_env: "STRAVA_CONFIG_FILE",
    default_config_file: "config.json",
    client_id_env: "STRAVA_CLIENT_ID",
    client_secret_env: "STRAVA_CLIENT_SECRET",
};

/// Configuration structure holding authentication-related constants and environment variable names.
struct StravaAuthConfig {
    /// URL for the OAuth authorization endpoint
    auth_url: &'static str,
    /// URL for the OAuth token endpoint
    token_url: &'static str,
    /// Environment variable name for the config file path
    config_file_env: &'static str,
    /// Default configuration file name
    default_config_file: &'static str,
    /// Environment variable name for the client ID
    client_id_env: &'static str,
    /// Environment variable name for the client secret
    client_secret_env: &'static str,
}

/// Authenticates with the Strava API using OAuth2 flow.
///
/// This function performs the following steps:
/// 1. Retrieves client ID and secret from environment variables
/// 2. Creates an authentication configuration
/// 3. Initiates the OAuth2 authorization process
///
/// # Returns
/// - `Ok(String)` - The access token for authenticated requests
/// - `Err(StravaError)` - If authentication fails or configuration is missing
///
/// # Errors
/// This function will return a `StravaError`:
/// - `StravaError::Config` if required environment variables are not set:
///   - `STRAVA_CLIENT_ID`
///   - `STRAVA_CLIENT_SECRET`
/// - `StravaError::Authentication` if the OAuth2 flow fails
///
pub fn auth_strava() -> Result<String, StravaError> {
    let client_id = env::var(AUTH_CONFIG.client_id_env).map_err(|_| StravaError::Config {
        message: "Missing client ID".to_string(),
        help: format!("Set the {} environment variable", AUTH_CONFIG.client_id_env),
    })?;

    let client_secret =
        env::var(AUTH_CONFIG.client_secret_env).map_err(|_| StravaError::Config {
            message: "Missing client secret".to_string(),
            help: format!(
                "Set the {} environment variable",
                AUTH_CONFIG.client_secret_env
            ),
        })?;

    let config = auth::Config::new(
        client_id,
        client_secret,
        String::new(),
        AUTH_CONFIG.auth_url.to_string(),
        AUTH_CONFIG.token_url.to_string(),
    );
    auth::get_authorization(config).map_err(|e| StravaError::Authentication {
        source: e.to_string().into(),
        help: Some("Check your Strava credentials and try again".to_string()),
    })
}

/// Retrieves the authenticated athlete's profile information from Strava.
///
/// # Returns
///
/// Returns a `Result` containing the athlete's profile information if successful,
/// or a `StravaError` if the operation fails.
///
/// # Errors
///
/// This function will return an error if:
/// - Authentication fails
/// - The API request fails
/// - The response cannot be parsed
pub fn get_authenticated_athlete() -> Result<AthleteCollection> {
    let access_token = obtain_access_token().wrap_err("Failed to obtain access token")?;

    athlete::get_athlete(&access_token)
        .map_err(|e| StravaError::Api {
            message: "Failed to get athlete information".to_string(),
            src: Some(e.to_string()),
        })
        .into_diagnostic()
}

/// Retrieves statistics for the authenticated Strava athlete.
///
/// This function fetches various statistics for the athlete, including total distance,
/// ride counts, run counts, and other activity metrics from the Strava API.
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `AthleteStats` - The athlete's statistics
/// * `StravaError` - Error if the operation fails
///
/// # Errors
///
/// This function will return an error if:
/// * Authentication fails during access token retrieval
/// * The API request to get athlete stats fails
pub fn get_athlete_stats() -> Result<AthleteStats> {
    let access_token = obtain_access_token().wrap_err("Failed to obtain access token")?;
    let athlete_id = get_authenticated_athlete()
        .wrap_err("Failed to get athlete ID")?
        .id;

    athlete::get_athlete_stats(&access_token, &athlete_id.to_string())
        .map_err(|e| StravaError::Api {
            message: "Failed to get athlete stats".to_string(),
            src: Some(e.to_string()),
        })
        .into_diagnostic()
}

/// Updates the authenticated athlete's weight in Strava.
///
/// # Arguments
///
/// * `weight` - The athlete's weight in kilograms as a string
///
/// # Returns
///
/// Returns a `Result` containing the status of the update operation if successful,
/// or a `StravaError` if the operation fails.
///
/// # Errors
///
/// This function will return an error if:
/// - Authentication fails
/// - The weight value is invalid
/// - The API request fails
pub fn update_athlete_weight(weight: &str) -> Result<String> {
    let access_token =
        obtain_access_token().wrap_err("Failed to obtain access token for weight update")?;

    athlete::update_athlete_weight(&access_token, weight)
        .map(|response| response.status().to_string())
        .map_err(|e| StravaError::Api {
            message: "Failed to update athlete weight".to_string(),
            src: Some(e.to_string()),
        })
        .into_diagnostic()
}

/// Obtains an access token for Strava API operations.
///
/// # Returns
///
/// Returns a `Result` containing the access token if successful,
/// or a `StravaError` if the operation fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The configuration file cannot be read
/// - The environment variables are not set
/// - The authentication process fails
fn obtain_access_token() -> Result<String> {
    let config_file = env::var(AUTH_CONFIG.config_file_env)
        .unwrap_or_else(|_| AUTH_CONFIG.default_config_file.to_string());

    get_access_token(&config_file).wrap_err("Failed to get access token")
}

/// Retrieves an access token using the provided configuration file.
///
/// # Arguments
///
/// * `config_file` - Path to the configuration file
///
/// # Returns
///
/// Returns a `Result` containing the access token if successful,
/// or a `StravaError` if the operation fails.
///
/// # Errors
///
/// This function will return an error if:
/// - Required environment variables are missing
/// - The configuration file is invalid
/// - The authentication process fails
fn get_access_token(config_file: &str) -> Result<String> {
    let client_id = env::var(AUTH_CONFIG.client_id_env).map_err(|_| StravaError::Config {
        message: "Missing client ID".to_string(),
        help: format!("Set the {} environment variable", AUTH_CONFIG.client_id_env),
    })?;

    let client_secret =
        env::var(AUTH_CONFIG.client_secret_env).map_err(|_| StravaError::Config {
            message: "Missing client secret".to_string(),
            help: format!(
                "Set the {} environment variable",
                AUTH_CONFIG.client_secret_env
            ),
        })?;

    let mut config = auth::Config::new(
        client_id,
        client_secret,
        String::new(),
        AUTH_CONFIG.auth_url.to_string(),
        AUTH_CONFIG.token_url.to_string(),
    );

    let token = if Path::new(config_file).exists() {
        config.refresh_token = Some(auth_config::config_file::load_config().refresh_token);
        auth::get_refresh_token(config)
    } else {
        auth::get_authorization(config)
    };

    token
        .map(|t| t.to_string())
        .map_err(|e| StravaError::Authentication {
            source: e.into(),
            help: Some("Check your credentials and network connection".to_string()),
        })
        .into_diagnostic()
}

/// Synchronizes the athlete's weight with Strava.
///
/// # Arguments
///
/// * `weight_in_kgs` - Optional weight value in kilograms
///
/// # Returns
///
/// Returns a `Result` indicating success or failure of the synchronization.
///
/// # Errors
///
/// This function will return an error if:
/// - The weight value is None
/// - The weight update operation fails
/// - Authentication fails
pub fn sync_weight_to_strava(weight_in_kgs: Option<String>) -> Result<()> {
    let weight = weight_in_kgs.ok_or_else(|| StravaError::Api {
        message: "Weight value is required".to_string(),
        src: None,
    })?;

    println!("Syncing to Strava...");
    update_athlete_weight(&weight).wrap_err("Failed to sync weight with Strava")?;
    println!("Weight updated in Strava to {} kg", weight);

    Ok(())
}
