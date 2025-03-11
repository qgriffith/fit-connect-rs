use crate::modules::strava;
use crate::utils::get_and_format_weight;
use clap::{Parser, Subcommand, ValueEnum};
use colored_json::to_colored_json_auto;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    ///Optional to enable logging
    #[arg(short, long)]
    log: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, ValueEnum)]
enum StatsOption {
    /// Get all athlete stats
    All,
    /// Get year-to-date run stats only
    YtdRun,
    /// Get your YTD Run in miles only
    YtdRunMiles,
    /// Get year-to-date ride stats only
    YtdRide,
    /// Get year-to-date swim stats only
    YtdSwim,
    /// Get your last 4 weeks run stats only
    RecentRun,
    /// Get your last 4 weeks of run in miles only
    RecentRunMiles,
    /// Get your last 4 weeks swim stats only,
    RecentSwim,
    ///Get your last 4 weeks ride stats only,
    RecentRide,
}

#[derive(Subcommand)]
enum Commands {
    Withings {
        #[arg(short, long)]
        last_weight: i64,
        #[arg(short, long)]
        strava_sync: bool,
    },
    Strava {
        #[arg(
            short = 'r',
            long,
            long_help = "Register with Strava needs to be run first"
        )]
        register: bool,
        #[arg(short = 'a', long)]
        get_athlete: bool,
        #[arg(short = 's', long, value_name = "OPTION")]
        get_stats: Option<StatsOption>,
    },
}

pub fn cli() {
    let cli = Cli::parse();

    if cli.log {
        simple_logger::SimpleLogger::new().env().init().unwrap();
    }

    match cli.command {
        Some(Commands::Withings {
            last_weight,
            strava_sync,
        }) => {
            let weight_in_kgs = get_and_format_weight(last_weight);
            println!("weight: {:?}", weight_in_kgs);
            println!("strava_sync: {:?}", strava_sync);
            if strava_sync {
                strava::sync_weight_to_strava(weight_in_kgs).expect("TODO: panic message");
            }
        }
        Some(Commands::Strava {
            register,
            get_athlete,
            get_stats,
        }) => {
            if register {
                strava::auth_strava().unwrap();
            }
            if get_athlete {
                let athlete = strava::get_authenticated_athlete().unwrap();
                let j = to_colored_json_auto(&athlete);
                println!("{}", j.unwrap());
            }
            if let Some(stats_option) = get_stats {
                match stats_option {
                    StatsOption::All => {
                        let stats = strava::get_athlete_stats().unwrap();
                        let j = to_colored_json_auto(&stats);
                        println!("{}", j.unwrap());
                    }
                    StatsOption::YtdRun => {
                        let stats = strava::get_athlete_stats().unwrap();
                        let j = to_colored_json_auto(&stats.ytd_run_totals);
                        println!("{}", j.unwrap());
                    }
                    StatsOption::YtdRunMiles => {
                        let stats = strava::get_athlete_stats().unwrap();
                        let miles = stats.ytd_run_totals.distance_in_miles();
                        println!("{:.2}", miles);
                    }
                    StatsOption::YtdRide => {
                        let stats = strava::get_athlete_stats().unwrap();
                        let j = to_colored_json_auto(&stats.ytd_ride_totals);
                        println!("{}", j.unwrap());
                    }
                    StatsOption::YtdSwim => {
                        let stats = strava::get_athlete_stats().unwrap();
                        let j = to_colored_json_auto(&stats.ytd_swim_totals);
                        println!("{}", j.unwrap());
                    }
                    StatsOption::RecentRun => {
                        let stats = strava::get_athlete_stats().unwrap();
                        let j = to_colored_json_auto(&stats.recent_run_totals);
                        println!("{}", j.unwrap());
                    }
                    StatsOption::RecentRunMiles => {
                        let stats = strava::get_athlete_stats().unwrap();
                        let miles = stats.recent_run_totals.distance_in_miles();
                        println!("{:.2}", miles);
                    }
                    StatsOption::RecentSwim => {
                        let stats = strava::get_athlete_stats().unwrap();
                        let j = to_colored_json_auto(&stats.recent_swim_totals);
                        println!("{}", j.unwrap());
                    }
                    StatsOption::RecentRide => {
                        let stats = strava::get_athlete_stats().unwrap();
                        let j = to_colored_json_auto(&stats.recent_ride_totals);
                        println!("{}", j.unwrap());
                    }
                }
            }
        }
        None => {
            println!("No command specified");
        }
    }
}
