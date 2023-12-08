use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let output = part1("day-01/src/input.txt");
    dbg!(output);
}

const DIGITS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
const DIGITS_AS_STRINGS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part1(file_path: &str) -> u32 {
    println!("{}", file_path);

    let mut result = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(string) = line {
                let mut first_number: u32 = 0;
                let mut first_number_index: usize = string.len();
                let mut last_number: u32 = 0;
                let mut last_number_index: usize = 0;

                for (char_index, ch) in DIGITS.iter().enumerate() {
                    let first_found_index_option = string.find(*ch).ok_or(0);
                    if let Ok(found_index) = first_found_index_option {
                        if found_index < first_number_index {
                            first_number_index = found_index;
                            first_number = u32::try_from(char_index + 1).unwrap();
                        }
                    }

                    let last_found_index_option = string.rfind(*ch).ok_or(0);
                    if let Ok(found_index) = last_found_index_option {
                        if found_index > last_number_index {
                            last_number_index = found_index;
                            last_number = u32::try_from(char_index + 1).unwrap();
                        }
                    }
                }

                for (digit_string_index, digit_string) in DIGITS_AS_STRINGS.iter().enumerate() {
                    let first_found_index_option = string.find(*digit_string).ok_or(0);
                    if let Ok(found_index) = first_found_index_option {
                        if found_index < first_number_index {
                            first_number_index = found_index;
                            first_number = u32::try_from(digit_string_index + 1).unwrap();
                        }
                    }

                    let last_found_index_option = string.rfind(*digit_string).ok_or(0);
                    if let Ok(found_index) = last_found_index_option {
                        if found_index > last_number_index {
                            last_number_index = found_index;
                            last_number = u32::try_from(digit_string_index + 1).unwrap();
                        }
                    }
                }

                println!("{} {} {}", string, first_number, last_number);

                result += (first_number * 10) + last_number;
            }
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("src/bin/input1.txt");
        assert_eq!(result, 142);
    }
}
