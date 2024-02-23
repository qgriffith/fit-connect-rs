use clap::{arg, Command};

pub(crate) fn cli() -> Command {
    Command::new("fitness-connect")
        .about("A sync tool for various fitness apps")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("withings")
                .about("Get Data from Withings")
                .arg(arg!(<LAST> "Integer: Day to get weight. 1 eq current day, 2 the day prior etc...").value_parser(clap::value_parser!(i64)))
                .arg(arg!(<STRAVA> "Sync to Strava")
                )
        )
}