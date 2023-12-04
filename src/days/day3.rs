use crate::days::Day;
use clap::{arg, command, value_parser};
use itertools::{enumerate};
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

fn run(input: String) -> Result<u32, Box<dyn std::error::Error>> {
    let schematic_lines = input
        .lines()
        // TODO avoid use of unwrap here
        .map(|line| {
            (
                line.to_string(),
                find_schematic_parts(line.to_string()).unwrap(),
            )
        })
        .collect::<Vec<(String, Vec<SchematicPart>)>>();

    // collect the number spans that are adjacent to a symbol span
    // uses enumerate to get the line number so that we can check the line above and below
    // the implementation preserves the original 2d vector structure so that we can push
    // numbers that are adjacent into the current line only to avoid duplication
    // an alternative implementation would be to hash the number spans by their line number
    // and store the adjacent numbers in a set
    let mut numbers_adjacent_to_symbols: Vec<Vec<&SchematicPart>> = Vec::new();
    for (schematic_idx, (_raw_line, parts)) in enumerate(&schematic_lines) {
        let mut numbers_adjacent_to_symbols_line: Vec<&SchematicPart> = Vec::new();
        for (part_idx, part) in enumerate(parts) {
            if let SchematicPart::NumberSpan(number_span) = part {
                println!(
                    "[{}] Number Span: {:?} {:?}",
                    schematic_idx, number_span.span, number_span.value
                );

                // check symbol before
                if part_idx > 0 {
                    // symbol before: +42
                    if let SchematicPart::SymbolSpan(symbol_span) = &parts[part_idx - 1] {
                        if symbol_span.span.end == number_span.span.start {
                            println!(
                                "[{}] symbol before: {:?} {:?}",
                                schematic_idx, symbol_span.char, number_span.value
                            );
                            numbers_adjacent_to_symbols_line.push(part);
                            continue;
                        }
                    }
                }

                // check symbol after
                if part_idx < parts.len() - 1 {
                    // symbol after: 42+
                    if let SchematicPart::SymbolSpan(symbol_span) = &parts[part_idx + 1] {
                        if symbol_span.span.start == number_span.span.end {
                            println!(
                                "[{}] symbol after: {:?} {:?}",
                                schematic_idx, number_span.value, symbol_span.char
                            );
                            numbers_adjacent_to_symbols_line.push(part);
                            continue;
                        }
                    }
                }

                // check the line above
                if schematic_idx > 0 {
                    let line_above = &schematic_lines[schematic_idx - 1].0;
                    // we need to expand the number span to include the characters that are diagonally adjacent
                    let expanded_number_span = expand_range(number_span);
                    // slice the line above to get the characters that contain symbols
                    // that are adjacent to this number
                    let line_above_slice = &line_above[expanded_number_span];
                    // if the line above contains any character that is not a '.' or a digit
                    // then it contains a symbol that is adjacent to this number
                    if line_above_slice
                        .chars()
                        .any(|c| !c.is_ascii_digit() && c != '.')
                    {
                        println!(
                            "[{}] symbol above: {:?} {:?}",
                            schematic_idx, line_above_slice, number_span.value
                        );
                        numbers_adjacent_to_symbols_line.push(part);
                        continue;
                    }
                }

                // check the line below
                if schematic_idx < schematic_lines.len() - 1 {
                    let line_below = &schematic_lines[schematic_idx + 1].0;
                    // we need to expand the number span to include the characters that are diagonally adjacent
                    let expanded_number_span = expand_range(number_span);
                    // slice the line above to get the characters that contain symbols
                    // that are adjacent to this number
                    let line_below_slice = &line_below[expanded_number_span];
                    // if the line above contains any character that is not a '.' or a digit
                    // then it contains a symbol that is adjacent to this number
                    if line_below_slice
                        .chars()
                        .any(|c| !c.is_ascii_digit() && c != '.')
                    {
                        println!(
                            "[{}] symbol above: {:?} {:?}",
                            schematic_idx, line_below_slice, number_span.value
                        );
                        numbers_adjacent_to_symbols_line.push(part);
                        continue;
                    }
                }
            }
        }
        numbers_adjacent_to_symbols.push(numbers_adjacent_to_symbols_line);
    }

    let sum: u32 = numbers_adjacent_to_symbols
        .iter()
        .flatten()
        .map(|part| match part {
            // only sum the values of the number spans
            SchematicPart::NumberSpan(span) => span.value,
            _ => 0,
        })
        .sum();
    Ok(sum)
}

fn expand_range(number_span: &NumberSpan) -> Range<usize> {
    if number_span.span.start == 0 {
        // range may be negative, but usize cannot so we need to handle this case
        // to avoid a subtract overflow panic
        number_span.span.start..number_span.span.end + 1
    } else {
        number_span.span.start - 1..number_span.span.end + 1
    }
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

    #[test]
    fn day3_sample() {
        let input = read_to_string("tests/day3").unwrap();
        let result = run(input).unwrap();
        assert_eq!(result, 4361);
    }

    #[test]
    fn day3_edge_case() {
        let input = read_to_string("tests/day3_edge_case").unwrap();
        let result = run(input).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn day3_edge_case_2() {
        let input = read_to_string("tests/day3_edge_case_2").unwrap();
        let result = run(input).unwrap();
        assert_eq!(result, 0);
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
