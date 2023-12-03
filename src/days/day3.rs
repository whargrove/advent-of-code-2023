use crate::days::Day;
use clap::{arg, command, value_parser};
use std::fs::read_to_string;
use std::ops::Range;
use std::path::PathBuf;

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
enum SchematicPart {
    NumberSpan(NumberSpan),
    SymbolSpan(SymbolSpan),
}

#[derive(Debug, PartialEq)]
struct NumberSpan {
    span: Range<usize>,
    value: u32,
}

#[derive(Debug, PartialEq)]
struct SymbolSpan {
    span: Range<usize>,
    char: char,
}

fn find_schematic_parts(input: String) -> Result<Vec<SchematicPart>, Box<dyn std::error::Error>> {
    let mut result: Vec<SchematicPart> = Vec::new();
    let mut buf: Vec<u32> = Vec::new();
    let mut start_idx = None;
    for (idx, char) in input.char_indices() {
        if char.is_ascii_digit() {
            buf.push(char.to_digit(10).unwrap());
            // only set start index if it is none to avoid moving forward the start of the span
            // for subsequent digits in the number
            if start_idx.is_none() {
                start_idx = Some(idx);
            }
            continue;
        }

        if char == '.' && buf.is_empty() {
            // the buffer is empty, so there's no number to fold and push into the result
            continue;
        }

        // if not a digit and the buffer is not empty, then fold the buffer into a single number
        // and push it into the result
        if !buf.is_empty() {
            let end_idx = idx;
            let value = buf.iter().fold(0, |acc, x| acc * 10 + x);
            let span = start_idx.unwrap()..end_idx;
            result.push(SchematicPart::NumberSpan(NumberSpan { span, value }));
            // reset the buffer and start index
            buf.clear();
            start_idx = None;
        }

        if char != '.' {
            // if not a digit and not a period, then it's a symbol
            let span = idx..idx + 1;
            result.push(SchematicPart::SymbolSpan(SymbolSpan { span, char }));
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    // #[test]
    fn day3_sample() {
        let input = read_to_string("tests/day3").unwrap();
        let result = run(input).unwrap();
        assert_eq!(result, 4361);
    }

    macro_rules! parameterized_schema_parts_test {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() -> Result<(), Box<dyn std::error::Error>> {
                let input = String::from($input);
                let result = find_schematic_parts(input)?;
                assert_eq!(result, $expected);
                Ok(())
            }
        };
    }

    parameterized_schema_parts_test!(
        day3_find_schematic_parts_number_spans,
        "467..114..",
        vec![
            SchematicPart::NumberSpan(NumberSpan {
                span: 0..3,
                value: 467
            }),
            SchematicPart::NumberSpan(NumberSpan {
                span: 5..8,
                value: 114
            }),
        ]
    );

    parameterized_schema_parts_test!(
        day3_find_schematic_parts_symbol_spans,
        "......#...",
        vec![SchematicPart::SymbolSpan(SymbolSpan {
            span: 6..7,
            char: '#'
        }),]
    );

    parameterized_schema_parts_test!(day3_find_schematic_parts_empty, "..........", vec![]);

    parameterized_schema_parts_test!(
        day3_find_schematic_parts_symbol_adjacent_to_number,
        "617*......",
        vec![
            SchematicPart::NumberSpan(NumberSpan {
                span: 0..3,
                value: 617
            }),
            SchematicPart::SymbolSpan(SymbolSpan {
                span: 3..4,
                char: '*'
            }),
        ]
    );
}
