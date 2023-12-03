use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::read_to_string;
use std::path::PathBuf;

use clap::{arg, command, value_parser};

use crate::Day;

pub struct Day2;

struct Set {
    pulls: Vec<Pull>,
}

struct Pull {
    color: Color,
    count: u32,
}

impl From<&str> for Pull {
    fn from(s: &str) -> Self {
        let mut each = s.trim_start().split(' ');
        // TODO Implement TryFrom instead of From to handle invalid inputs
        let count = each.next().unwrap().parse::<u32>().unwrap();
        let color = each.next().unwrap().into();
        Pull { color, count }
    }
}

enum Color {
    Red,
    Green,
    Blue,
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            color => panic!("Invalid color: {}", color),
        }
    }
}

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

impl Day for Day2 {
    fn command() -> clap::Command {
        command!("day2").arg(arg!(--"input" <PATH>).value_parser(value_parser!(PathBuf)))
    }

    fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
        let input = match matches.get_one::<PathBuf>("input") {
            Some(input) => read_to_string(input)?,
            None => panic!("Input file is missing"),
        };

        let result = run(input)?;
        println!("{}", result);
        Ok(())
    }
}

fn run(input: String) -> Result<u32, Box<dyn Error>> {
    let sum = input
        .lines()
        // Map each line to a tuple of Game ID and a Vector of Pulls
        // Each Pull contains a count and a color
        .map(|game_line| {
            let mut game_split = game_line.split(':');
            let game_id = game_split
                .next()
                .ok_or(AdventOfCodeError::InvalidInput)?
                .trim_start_matches("Game ")
                .parse::<u32>()?;
            let mut pulls = Vec::new();
            let set_split = game_split
                .next()
                .ok_or(AdventOfCodeError::InvalidInput)?
                .split(';');
            for set in set_split {
                set.split(',')
                    .map(|l| l.into())
                    .for_each(|pull| pulls.push(pull));
            }
            Ok::<(u32, Set), AdventOfCodeError>((game_id, Set { pulls }))
        })
        .filter_map(Result::ok)
        // Filter to only games where all pulls from the sets have a count that is less than or equal to the max
        .filter(|(_, set)| {
            set.pulls.iter().all(|pull| match pull.color {
                Color::Red => pull.count <= RED_MAX,
                Color::Green => pull.count <= GREEN_MAX,
                Color::Blue => pull.count <= BLUE_MAX,
            })
        })
        // Sum the game IDs of the remaining games
        .map(|(game_id, _)| game_id)
        .sum::<u32>();
    Ok(sum)
}

#[derive(Debug)]
enum AdventOfCodeError {
    InvalidInput,
}

impl Display for AdventOfCodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AdventOfCodeError::InvalidInput => write!(f, "Invalid input"),
        }
    }
}

impl Error for AdventOfCodeError {}

impl From<std::num::ParseIntError> for AdventOfCodeError {
    fn from(_: std::num::ParseIntError) -> Self {
        AdventOfCodeError::InvalidInput
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = read_to_string(PathBuf::from("tests/day2")).unwrap();
        let result = run(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 8);
    }
}
