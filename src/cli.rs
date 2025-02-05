use crate::modules::strava;
use crate::utils::get_and_format_weight;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Withings {
        #[clap(short, long)]
        last_weight: i64,
        #[clap(short, long)]
        strava_sync: bool,
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
        None => {
            println!("No command specified");
        }
    }
}
