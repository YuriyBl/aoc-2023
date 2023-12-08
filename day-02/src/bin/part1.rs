use regex::Regex;
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
    let output = part1("day-02/src/input.txt");
    dbg!(output);
}

fn part1(file_path: &str) -> i32 {
    println!("{}", file_path);

    let num_of_reds = 12;
    let num_of_blues = 13;
    let num_of_greens = 14;

    let mut result = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(game_string) = line {
                let cubes_re = Regex::new(r"(?<num>[0-9]*) (?<color>(red|blue|green))").unwrap();
                let game_re = Regex::new(r"Game (?<num>[0-9]*):").unwrap();

                let game_number_string: &str =
                    &game_re.captures(game_string.as_str()).unwrap()["num"];
                let game_number = game_number_string.parse::<i32>().unwrap();

                let mut game_possible = true;

                println!("game_number: {}", game_number);

                for (pos, string_part) in game_string.split(';').enumerate() {
                    println!("item: {}", string_part);

                    cubes_re.captures_iter(string_part).for_each(|caps| {
                        let number_str = caps.name("num").unwrap().as_str();
                        let number = number_str.parse::<i32>().unwrap();
                        let color = caps.name("color").unwrap().as_str();

                        if (color == "red" && number > num_of_reds)
                            || (color == "green" && number > num_of_greens)
                            || (color == "blue" && number > num_of_blues)
                        {
                            game_possible = false;
                        };

                        println!("number: {}; color: {};", number, color);
                    })
                }

                if game_possible {
                    result += game_number;
                }
            }
        }
    }

    return result;
}
