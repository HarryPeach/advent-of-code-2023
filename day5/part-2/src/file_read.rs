use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::map_manager::{MapManager, MapTypes};

pub fn load_file(map_manager: &mut MapManager, filename: &str) -> Vec<(i64, i64)> {
    let mut map_type: MapTypes = MapTypes::SeedToSoil;
    let mut seeds: Option<Vec<(i64, i64)>> = None;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(valid_line) = line {
                match valid_line.trim() {
                    "seed-to-soil map:" => {
                        map_type = MapTypes::SeedToSoil;
                    }
                    "soil-to-fertilizer map:" => {
                        map_type = MapTypes::SoilToFertilizier;
                    }
                    "fertilizer-to-water map:" => {
                        map_type = MapTypes::FertilizerToWater;
                    }
                    "water-to-light map:" => {
                        map_type = MapTypes::WaterToLight;
                    }
                    "light-to-temperature map:" => {
                        map_type = MapTypes::LightToTemp;
                    }
                    "temperature-to-humidity map:" => {
                        map_type = MapTypes::TempToHumidity;
                    }
                    "humidity-to-location map:" => {
                        map_type = MapTypes::HumidityToLocation;
                    }
                    "" => {
                        // no-op
                    }
                    _ => {
                        if valid_line.starts_with("seeds: ") {
                            let seed_line = valid_line.replace("seeds: ", "");
                            let raw_seeds = seed_line
                                .split(" ")
                                .map(|x| x.parse::<i64>().unwrap())
                                .collect::<Vec<_>>();
                            let mut seed_vec: Vec<(i64, i64)> = vec![];
                            for i in 0..raw_seeds.len() / 2 {
                                seed_vec.push((raw_seeds[i * 2], raw_seeds[(i * 2) + 1]));
                            }
                            seeds = Some(seed_vec.to_owned());
                        } else {
                            map_manager.load_mapping_from_line(&valid_line, &map_type);
                        }
                    }
                }
            }
        }
    }
    seeds.expect("Seeds were read incorrectly!")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
