#[derive(Debug)]
pub struct Transform {
    start: i64,
    end: i64,
    modifier: i64,
}

pub enum MapTypes {
    SeedToSoil,
    SoilToFertilizier,
    FertilizerToWater,
    WaterToLight,
    LightToTemp,
    TempToHumidity,
    HumidityToLocation,
}

pub struct MapManager {
    pub seed_to_soil_ranges: Vec<Transform>,
    pub soil_to_fertilizer_ranges: Vec<Transform>,
    pub fertilizer_to_water_ranges: Vec<Transform>,
    pub water_to_light_ranges: Vec<Transform>,
    pub light_to_temp_ranges: Vec<Transform>,
    pub temp_to_humidity_ranges: Vec<Transform>,
    pub humidity_to_location_ranges: Vec<Transform>,
}

impl MapManager {
    pub fn new() -> MapManager {
        MapManager {
            seed_to_soil_ranges: vec![],
            soil_to_fertilizer_ranges: vec![],
            fertilizer_to_water_ranges: vec![],
            water_to_light_ranges: vec![],
            light_to_temp_ranges: vec![],
            temp_to_humidity_ranges: vec![],
            humidity_to_location_ranges: vec![],
        }
    }

    pub fn load_mapping_from_line(&mut self, line: &str, map: &MapTypes) {
        let splits = line
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        // We assume that there are only ever 3 values in this line
        assert!(
            splits.len() == 3,
            "There were more than three values in the input line"
        );

        let transform = Transform {
            start: splits[1],
            end: splits[1] + splits[2] - 1,
            modifier: splits[0] - splits[1],
        };

        match map {
            MapTypes::SeedToSoil => self.seed_to_soil_ranges.push(transform),
            MapTypes::SoilToFertilizier => self.soil_to_fertilizer_ranges.push(transform),
            MapTypes::FertilizerToWater => self.fertilizer_to_water_ranges.push(transform),
            MapTypes::WaterToLight => self.water_to_light_ranges.push(transform),
            MapTypes::LightToTemp => self.light_to_temp_ranges.push(transform),
            MapTypes::TempToHumidity => self.temp_to_humidity_ranges.push(transform),
            MapTypes::HumidityToLocation => self.humidity_to_location_ranges.push(transform),
        }
    }

    pub fn get_mapping(&self, value: i64, ranges: MapTypes) -> i64 {
        let new_ranges = match ranges {
            MapTypes::SeedToSoil => &self.seed_to_soil_ranges,
            MapTypes::SoilToFertilizier => &self.soil_to_fertilizer_ranges,
            MapTypes::FertilizerToWater => &self.fertilizer_to_water_ranges,
            MapTypes::WaterToLight => &self.water_to_light_ranges,
            MapTypes::LightToTemp => &self.light_to_temp_ranges,
            MapTypes::TempToHumidity => &self.temp_to_humidity_ranges,
            MapTypes::HumidityToLocation => &self.humidity_to_location_ranges,
        };

        for range in new_ranges {
            if value <= range.end && value >= range.start {
                return value + range.modifier;
            }
        }
        value
    }
}
