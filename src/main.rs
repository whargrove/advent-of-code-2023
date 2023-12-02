use clap::Command;
use day1::Day1;

mod day1;

trait Day {
    fn command() -> Command;
    fn run(matches: &clap::ArgMatches);
}

fn main() {
    let cmd = Command::new("advent-of-code-2023")
        .bin_name("advent-of-code-2023")
        .subcommand_required(true)
        .subcommand(Day1::command());
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("day1", matches)) => Day1::run(matches),
        _ => panic!("Sub-command is not yet implemented"),
    };
}
