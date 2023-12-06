use rayon::prelude::*;

use map_manager::MapTypes;

use crate::file_read::load_file;
use crate::map_manager::MapManager;

mod file_read;
mod map_manager;

fn get_location(seed: i64, map_manager: &MapManager) -> i64 {
    let soil_num = map_manager.get_mapping(seed, MapTypes::SeedToSoil);
    let fert_num = map_manager.get_mapping(soil_num, MapTypes::SoilToFertilizier);
    let water_num = map_manager.get_mapping(fert_num, MapTypes::FertilizerToWater);
    let light_num = map_manager.get_mapping(water_num, MapTypes::WaterToLight);
    let temp_num = map_manager.get_mapping(light_num, MapTypes::LightToTemp);
    let humid_num = map_manager.get_mapping(temp_num, MapTypes::TempToHumidity);
    map_manager.get_mapping(humid_num, MapTypes::HumidityToLocation)
}

fn main() {
    let mut map_manager = MapManager::new();

    let seeds_and_ranges = load_file(&mut map_manager, "data.txt");

    let collapsed_seeds =  seeds_and_ranges.par_iter().flat_map(|tup| {
        let mut x: Vec<i64> = vec![];
        for item in tup.0..(tup.0 + tup.1) {
            x.push(item);
        }
        return x;
    }).collect::<Vec<_>>();

    let lowest_location = collapsed_seeds.par_iter().fold(|| -1, |acc, seed| {
        let new_location = get_location(*seed, &map_manager);
        if new_location < acc || acc == -1 {
            return new_location;
        }
        return acc;
    }).min();
    println!("lowest loc: {:?}", lowest_location.unwrap());
}
