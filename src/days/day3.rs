use std::fs::read_to_string;
use std::ops::Range;
use std::path::PathBuf;
use clap::{arg, command, value_parser};
use crate::days::Day;

pub struct Day3;

impl Day for Day3 {
    fn command() -> clap::Command {
        command!("day3").arg(arg!(--"input" <PATH>).value_parser(value_parser!(PathBuf)))
    }

    fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
        let input = match matches.get_one::<PathBuf>("input") {
            Some(input) => read_to_string(input)?,
            None => panic!("Input file is missing"),
        };

        let result = run(input)?;
        println!("{}", result);
        Ok(())
    }
}

fn run(_input: String) -> Result<u32, Box<dyn std::error::Error>> {
    // for each line, find the discrete numbers. A number is discrete if it is not adjacent to another number.
    todo!()
}

#[derive(Debug, PartialEq)]
struct NumberSpan {
    span: Range<usize>,
    value: u32,
}

fn find_nums(input: String) -> Result<Vec<NumberSpan>, Box<dyn std::error::Error>> {
    let mut result: Vec<NumberSpan> = Vec::new();
    let mut buf: Vec<u32> = Vec::new();
    let mut start_idx = None;
    for (idx, char) in input.char_indices() {
        if char.is_ascii_digit() {
            buf.push(char.to_digit(10).unwrap());
            if start_idx.is_none() {
                start_idx = Some(idx);
            }
        } else if !buf.is_empty() {
            let end_idx = idx;
            // if not a digit and the buffer is not empty, then fold the buffer into a single number
            // and push it into the result.
            // TODO handle symbols
            let value = buf.iter().fold(0, |acc, x| acc * 10 + x);
            let span = start_idx.unwrap()..end_idx;
            result.push(NumberSpan { span, value });
            // reset
            buf.clear();
            start_idx = None;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;

    #[test]
    fn day3() {
        let input = read_to_string("tests/day3").unwrap();
        let result = run(input).unwrap();
        assert_eq!(result, 4361);
    }

    #[test]
    fn day3_find_num() {
        // just the first line
        let input = read_to_string("tests/day3").unwrap().lines().next().unwrap().to_owned();
        let result = find_nums(input).unwrap().iter().map(|x| x.value).collect::<Vec<u32>>();
        assert_eq!(result, vec![467, 114]);
    }

    #[test]
    fn day3_find_num_with_span() {
        // just the first line
        let input = read_to_string("tests/day3").unwrap().lines().next().unwrap().to_owned();
        let result = find_nums(input).unwrap();
        assert_eq!(result, vec![
            NumberSpan { span: 0..3, value: 467 },
            NumberSpan { span: 5..8, value: 114 },
        ]);
    }
}