use std::error::Error;
use std::{fs::read_to_string, path::PathBuf};

use crate::days::Day;
use clap::{arg, command, value_parser, ArgMatches, Command};

pub struct Day1;

impl Day for Day1 {
    fn command() -> Command {
        command!("day1").arg(arg!(--"input" <PATH>).value_parser(value_parser!(PathBuf)))
    }

    fn run(arg_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let input = match arg_matches.get_one::<PathBuf>("input") {
            Some(input) => read_to_string(input)?,
            None => panic!("Input file is missing"),
        };
        let calibration_sum: u32 = input
            .lines()
            .map(|line| {
                let mut chars = line.chars().peekable();
                let mut rev_chars = line.chars().rev().peekable();
                let mut first_dig = None;
                let mut last_dig = None;
                while let (Some(&first), Some(&last)) = (chars.peek(), rev_chars.peek()) {
                    if first.is_ascii_digit() {
                        first_dig = Some(first);
                    }
                    if last.is_ascii_digit() {
                        last_dig = Some(last);
                    }
                    if first_dig.is_some() && last_dig.is_some() {
                        break;
                    }
                    // only move the iterators if we haven't found a digit yet
                    if first_dig.is_none() {
                        chars.next();
                    }
                    if last_dig.is_none() {
                        rev_chars.next();
                    }
                }
                let first_dig = first_dig.and_then(|c| c.to_digit(10)).unwrap_or(0);
                let last_dig = last_dig.and_then(|c| c.to_digit(10)).unwrap_or(0);
                first_dig * 10 + last_dig
            })
            .sum();
        println!("Calibration sum: {}", calibration_sum);
        Ok(())
    }
}
