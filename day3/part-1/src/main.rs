use regex::Regex;
use std::cmp::min;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut file_lines: Vec<String> = vec![];
    let mut acc = 0;

    if let Ok(lines) = read_lines("data.txt") {
        for line in lines {
            if let Ok(valid_line) = line {
                file_lines.push(valid_line.to_owned());
            }
        }
    }

    for (index, line) in file_lines.iter().enumerate() {
        let locations = get_number_locations(line);
        for location in locations {
            if is_part_number(&file_lines, index, location.0, location.1) {
                acc += &line[location.0..location.1].parse::<i32>().unwrap();
            }
        }
    }
    println!("acc: {:?}", acc);
}

// This works by calculating a bounding box around the number that is detected, and then checking every character within the box to see if
// they are not a number or period
fn is_part_number(lines: &Vec<String>, line_no: usize, start: usize, end: usize) -> bool {
    let start_point: (usize, usize) = (
        start.saturating_sub(1),
        line_no.saturating_sub(1),
    );
    let end_point: (usize, usize) = (
        min(end, lines[line_no].len() - 1),
        min(line_no + 1, lines.len() - 1),
    );

    
    for y in start_point.1..(end_point.1 + 1) {
        for x in start_point.0..(end_point.0 + 1) {
            let current_char = lines[y].chars().nth(x).unwrap();
            if !['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.'].contains(&current_char) {
                return true;
            }
        }
    }

    false
}

// Get a list of locations of potential numbers within a line
// The result is a list of tuples, each value representing the first and last index
// of each number
fn get_number_locations(line: &str) -> Vec<(usize, usize)> {
    let mut locations: Vec<(usize, usize)> = vec![];
    let digit_regex = Regex::new(r"\d+").unwrap();
    let matches = digit_regex.find_iter(line);

    for match_ in matches {
        locations.push((match_.start(), match_.end()));
    }
    locations
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
