use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

/// Creates the last argument for weigh-in.
///
/// The `last` argument is used to specify the weigh-in date relative to the current day.
/// It can be set to `1` for the current day, `2` for the previous day, and so on.
///
/// # Returns
/// Returns an instance of `Arg` struct that represents the `last` argument.
fn create_last_arg() -> Arg {
    Arg::new("last")
        .short('l')
        .long("last")
        .help("Last weigh-in. Set to 1 for current day, 2 for the previous etc...")
        .action(ArgAction::Set)
        .default_value("1")
        .default_missing_value("1")
        .required(true)
        .num_args(0..=1)
        .value_parser(value_parser!(i64))
}

/// Creates a command line argument for enabling weight synchronization with Strava.
///
/// The `-s` or `--strava-sync` option can be used to enable weight synchronization with Strava.
/// By default, weight synchronization is disabled.
///
/// The `strava_sync_enabled` variable will be `true` if the `-s` or `--strava-sync` option is provided.
///
/// # Returns
///
/// The `Arg` struct representing the `--strava-sync` command line option.
fn create_strava_sync_arg() -> Arg {
    Arg::new("strava-sync")
        .long("strava-sync")
        .short('s')
        .help("Sync weight to strava")
        .action(ArgAction::SetTrue)
        .required(false)
}

/// Returns a `Command` object for the `fitness-connect` CLI.
///
/// # Return
///
/// Returns a `Command` object for the `fitness-connect` CLI.
pub(crate) fn cli() -> ArgMatches {
    Command::new("fitness-connect")
        .about("A sync tool for various fitness apps")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // Withings subcommand
        .subcommand(
            Command::new("withings")
                .short_flag('W')
                .long_flag("withings")
                .about("Get Data from Withings")
                .arg(create_last_arg())
                .arg(create_strava_sync_arg()),
        )
        .get_matches()
}
