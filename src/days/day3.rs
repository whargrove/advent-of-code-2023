use crate::days::Day;
use clap::{arg, command, value_parser};
use itertools::enumerate;
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
        .map(|line| find_schematic_parts(line.to_string()))
        .filter_map(Result::ok)
        .collect::<Vec<Vec<SchematicPart>>>();

    // collect the number spans that are adjacent to a symbol span
    // uses enumerate to get the line number so that we can check the line above and below
    // the implementation preserves the original 2d vector structure so that we can push
    // numbers that are adjacent into the current line only to avoid duplication
    // an alternative implementation would be to hash the number spans by their line number
    // and store the adjacent numbers in a set
    let mut numbers_adjacent_to_symbols: Vec<Vec<&SchematicPart>> = Vec::new();
    for (schematic_idx, parts) in enumerate(&schematic_lines) {
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

                // todo: refactor to expand this number span
                // and then check if the expanded span overlaps with any symbol spans from previous
                // or next lines.
                // we could use the raw lines here and slice the chars to check if overlap the expanded span

                // check the line above
                if schematic_idx > 0 {
                    let line_above = &schematic_lines[schematic_idx - 1];
                    for part_above in line_above {
                        if let SchematicPart::SymbolSpan(symbol_span) = part_above {
                            // a symbol on a previous or next line is adjacent to this number if
                            // the symbol is in a range defined by the span of the number +/- 1
                            // to allow diagonal adjacency
                            println!(
                                "[{}] Symbol Span: {:?} {:?}",
                                schematic_idx, symbol_span.span, symbol_span.char
                            );
                            let expanded_symbol_span = if symbol_span.span.start == 0 {
                                // fix subtract overflow panic
                                symbol_span.span.start..symbol_span.span.end + 1
                            } else {
                                symbol_span.span.start - 1..symbol_span.span.end + 1
                            };
                            println!(
                                "[{}] Expanded Symbol Span: {:?} {:?}",
                                schematic_idx, expanded_symbol_span, symbol_span.char
                            );
                            if expanded_symbol_span.contains(&number_span.span.start)
                                // fix bug where the end of the exclusive span should not be counted as overlapping
                                // with the expanded symbol span.
                                || expanded_symbol_span.contains(&(&number_span.span.end - 1))
                            {
                                println!(
                                    "[{}] symbol above: {:?} {:?}",
                                    schematic_idx, symbol_span.char, number_span.value
                                );
                                numbers_adjacent_to_symbols_line.push(part);
                                break;
                            }
                        }
                    }
                }

                // check the line below
                if schematic_idx < schematic_lines.len() - 1 {
                    let line_below = &schematic_lines[schematic_idx + 1];
                    for part_below in line_below {
                        if let SchematicPart::SymbolSpan(symbol_span) = part_below {
                            // a symbol on a previous or next line is adjacent to this number if
                            // the symbol is in a range defined by the span of the number +/- 1
                            // to allow diagonal adjacency
                            println!(
                                "[{}] Symbol Span: {:?} {:?}",
                                schematic_idx, symbol_span.span, symbol_span.char
                            );
                            let expanded_symbol_span = if symbol_span.span.start == 0 {
                                // fix subtract overflow panic
                                symbol_span.span.start..symbol_span.span.end + 1
                            } else {
                                symbol_span.span.start - 1..symbol_span.span.end + 1
                            };
                            println!(
                                "[{}] Expanded Symbol Span: {:?} {:?}",
                                schematic_idx, expanded_symbol_span, symbol_span.char
                            );
                            if expanded_symbol_span.contains(&number_span.span.start)
                                // fix bug where the end of the exclusive span should not be counted as overlapping
                                // with the expanded symbol span.
                                || expanded_symbol_span.contains(&(&number_span.span.end - 1))
                            {
                                println!(
                                    "[{}] symbol below: {:?} {:?}",
                                    schematic_idx, symbol_span.char, number_span.value
                                );
                                numbers_adjacent_to_symbols_line.push(part);
                                break;
                            }
                        }
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
