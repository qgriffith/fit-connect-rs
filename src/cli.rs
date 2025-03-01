use crate::modules::strava;
use crate::utils::get_and_format_weight;
use clap::{Parser, Subcommand};
use colored_json::to_colored_json_auto;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
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
        #[arg(short = 'a', long)]
        get_athlete: bool,
        #[arg(short = 's', long)]
        get_stats: bool,
    },
}

pub fn cli() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Withings {
            last_weight,
            strava_sync,
        }) => {
            let weight_in_kgs = get_and_format_weight(last_weight);
            println!("weight: {:?}", weight_in_kgs);
            println!("strava_sync: {:?}", strava_sync);
            if strava_sync {
                strava::sync_strava(weight_in_kgs);
            }
        }
        Some(Commands::Strava {
            get_athlete,
            get_stats,
        }) => {
            if get_athlete {
                let athlete = strava::get_authed_athlete();
                let j = to_colored_json_auto(&athlete);
                println!("{}", j.unwrap());
            }
            if get_stats {
                let athlete = strava::get_athlete_stats();
                let j = to_colored_json_auto(&athlete.unwrap());
                println!("{}", j.unwrap());
            }
        }
        None => {
            println!("No command specified");
        }
    }
}
