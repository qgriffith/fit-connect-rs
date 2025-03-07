use crate::modules::strava;
use crate::utils::get_and_format_weight;
use clap::{Parser, Subcommand};
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
        #[arg(short = 's', long)]
        get_stats: bool,
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
            if get_stats {
                let athlete = strava::get_authenticated_athlete().unwrap();
                let j = to_colored_json_auto(&athlete);
                println!("{}", j.unwrap());
            }
        }
        None => {
            println!("No command specified");
        }
    }
}
