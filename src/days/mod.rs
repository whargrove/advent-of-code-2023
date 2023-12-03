use std::error::Error;

use clap::{ArgMatches, Command};

pub mod day1;
pub mod day2;

pub trait Day {
    fn command() -> Command;
    fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>>;
}
