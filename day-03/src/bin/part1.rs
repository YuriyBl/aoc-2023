use regex::Regex;
use utils;

fn main() {
    let output = part1("day-03/src/input.txt");
    dbg!(output);
}

#[derive(Debug)]
struct Symbol {
    char: char,
    row: usize,
    column: usize,
}

fn part1(file_path: &str) -> u64 {
    let numbers_regex = Regex::new(r"(?<num>[0-9]+)").unwrap();

    let mut symbols: Vec<Symbol> = vec![];
    let mut lines_count: usize = 0;

    let mut result: u64 = 0;

    utils::process_file(file_path, |line: &str, line_index: usize| {
        lines_count += 1;
        for (char_index, char) in line.chars().enumerate() {
            if !char.is_alphanumeric() && char != '.' {
                symbols.push(Symbol {
                    char: char,
                    row: line_index,
                    column: char_index,
                });
            }
        }
    });

    utils::process_file(file_path, |line: &str, line_index: usize| {
        numbers_regex.captures_iter(line).for_each(|caps| {
            let cap_match = caps.name("num").unwrap();
            let number_str = cap_match.as_str();
            let number = number_str.parse::<u64>().unwrap();

            let start = cap_match.start();
            let end = cap_match.end();

            let symbol_row_start = if line_index == 0 { 0 } else { line_index - 1 };
            let symbol_row_end = if line_index + 1 == lines_count {
                lines_count
            } else {
                line_index + 1
            };
            let symbol_col_start = if start == 0 { 0 } else { start - 1 };
            let symbol_col_end = if end + 1 == line.len() {
                line.len() - 1
            } else {
                end
            };

            let mut has_touching_symbol = false;
            for symbol in &symbols {
                if symbol.row >= symbol_row_start
                    && symbol.row <= symbol_row_end
                    && symbol.column >= symbol_col_start
                    && symbol.column <= symbol_col_end
                {
                    has_touching_symbol = true;
                }
            }

            if has_touching_symbol {
                result += number;
            }
        })
    });

    return result;
}
