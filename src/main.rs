use crate::{days::day1::Day1, days::day2::Day2, days::day3::Day3, days::Day};
use clap::Command;
use std::error::Error;

mod days;

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = Command::new("advent-of-code-2023")
        .bin_name("advent-of-code-2023")
        .subcommand_required(true)
        .subcommand(Day1::command())
        .subcommand(Day2::command())
        .subcommand(Day3::command())
        ;
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("day1", matches)) => Day1::run(matches),
        Some(("day2", matches)) => Day2::run(matches),
        Some(("day3", matches)) => Day3::run(matches),
        _ => panic!("Sub-command is not yet implemented"),
    }?;
    Ok(())
}
