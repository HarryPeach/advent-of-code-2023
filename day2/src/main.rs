use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// This could've been an array, which would have made the whole program less verbose,
// but I quite like having the semantic information for each element included
struct Selection {
    red: i32,
    green: i32,
    blue: i32,
}

// The combination of cubes that the elf has asked us to search for
const GAME_QUERY: Selection = Selection {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() {
    let mut valid_games: Vec<i32> = vec![];
    let mut powers: Vec<f32> = vec![];

    if let Ok(lines) = read_lines("data.txt") {
        for (index, line) in lines.enumerate() {
            if let Ok(valid_line) = line {
                let mut game_valid_flag = true;
                let mut current_minimum = Selection {
                    red: 0,
                    green: 0,
                    blue: 0,
                };

                let colon_location = valid_line.find(":").expect("Could not find a colon in the parsed line, so the game title could not be stripped.");
                let string_to_parse = &valid_line[colon_location + 2..];

                let parts = string_to_parse.split(";");
                for part in parts {
                    let selection = selection_from_part(part.trim());
                    if (selection.blue > GAME_QUERY.blue)
                        || (selection.red > GAME_QUERY.red)
                        || (selection.green > GAME_QUERY.green)
                    {
                        game_valid_flag = false;
                    }
                    if selection.blue > current_minimum.blue {
                        current_minimum.blue = selection.blue;
                    }
                    if selection.red > current_minimum.red {
                        current_minimum.red = selection.red;
                    }
                    if selection.green > current_minimum.green {
                        current_minimum.green = selection.green;
                    }
                }

                powers.push(f32::cbrt(
                    i32::pow(current_minimum.red, 3) as f32
                        * i32::pow(current_minimum.green, 3) as f32
                        * i32::pow(current_minimum.blue, 3) as f32,
                ));

                if game_valid_flag {
                    valid_games.push(i32::try_from(index).unwrap() + 1);
                }
            }
        }
    }
    println!("Valid games: \n\t{:?}\n", valid_games);
    println!(
        "Sum of valid indexes: \n\t{}\n",
        valid_games.iter().sum::<i32>()
    );
    println!("List of powers: \n\t{:?}\n", powers);
    println!("Sum of powers: \n\t{}\n", powers.iter().sum::<f32>());
}

// Creates a selection from a "part" of the input, i.e. the bits between the semi-colons
fn selection_from_part(part: &str) -> Selection {
    Selection {
        red: extract_digits(r"\d+ red", part),
        green: extract_digits(r"\d+ green", part),
        blue: extract_digits(r"\d+ blue", part),
    }
}

/// Extracts digits from a string e.g. "6 red, 4 blue", will panic if the string contains none
fn extract_digits(regex_str: &str, part: &str) -> i32 {
    let constructed_regex = Regex::new(regex_str).unwrap();
    let digit_regex = Regex::new(r"\d+").unwrap();

    let digit = match constructed_regex.find(part) {
        Some(x) => digit_regex
            .find(x.as_str())
            .unwrap()
            .as_str()
            .parse()
            .unwrap(),
        None => 0,
    };

    digit
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
