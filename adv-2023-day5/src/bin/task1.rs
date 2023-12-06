use core::panic;
use std::collections::BTreeMap;

use adv_2023_common::Task;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

enum ParsedLine {
    Empty,
    Seeds(Vec<u32>),
    Map(MapType),
    Values(u32, u32, u32),
}

impl ParsedLine {
    pub fn parse(line: &str) -> Self {
        if line.is_empty() {
            return ParsedLine::Empty;
        }
        if line.ends_with(" map:") {
            return Self::parse_map(line);
        }
        if line.starts_with("seeds: ") {
            return Self::parse_seeds(line);
        }
        Self::parse_values(line)
    }

    fn parse_map(line: &str) -> Self {
        match line[..line.len() - 5].trim() {
            "seed-to-soil" => ParsedLine::Map(MapType::SeedToSoil),
            "soil-to-fertilizer" => ParsedLine::Map(MapType::SoilToFertilizer),
            "fertilizer-to-water" => ParsedLine::Map(MapType::FertilizerToWater),
            "water-to-light" => ParsedLine::Map(MapType::WaterToLight),
            "light-to-temperature" => ParsedLine::Map(MapType::LightToTemperature),
            "temperature-to-humidity" => ParsedLine::Map(MapType::TemperatureToHumidity),
            "humidity-to-location" => ParsedLine::Map(MapType::HumidityToLocation),
            map => panic!("Unknown map type: {}", map),
        }
    }

    fn parse_seeds(line: &str) -> Self {
        let mut nums = Vec::with_capacity(32);
        for num in line[7..].trim().split(' ') {
            nums.push(num.parse::<u32>().unwrap());
        }
        ParsedLine::Seeds(nums)
    }

    fn parse_values(line: &str) -> Self {
        let mut parts = line.split(' ');
        let a = parts.next().unwrap().parse::<u32>().unwrap();
        let b = parts.next().unwrap().parse::<u32>().unwrap();
        let c = parts.next().unwrap().parse::<u32>().unwrap();
        // swap destination and source for better lookup
        ParsedLine::Values(b, a, c)
    }
}

#[derive(Debug, Default)]
struct State {
    current_map: Option<MapType>,
    seeds: Vec<u32>,
    maps: BTreeMap<MapType, Vec<(u32, u32, u32)>>,
}

impl State {
    /// lookup from seed to closest location
    fn lookup(&self, seed: u32) -> u32 {
        let soil = self.lookup_map(MapType::SeedToSoil, seed);
        let fertilizer = self.lookup_map(MapType::SoilToFertilizer, soil);
        let water = self.lookup_map(MapType::FertilizerToWater, fertilizer);
        let light = self.lookup_map(MapType::WaterToLight, water);
        let temperature = self.lookup_map(MapType::LightToTemperature, light);
        let humidity = self.lookup_map(MapType::TemperatureToHumidity, temperature);
        self.lookup_map(MapType::HumidityToLocation, humidity)
    }

    fn lookup_map(&self, map_type: MapType, item: u32) -> u32 {
        let map = self.maps.get(&map_type).unwrap();
        match map.binary_search_by_key(&item, |(a, _, _)| *a) {
            Ok(pos) => map[pos].1,
            Err(pos) => {
                if pos == 0 {
                    item
                } else {
                    let (src, dst, len) = map[pos - 1];
                    if src + len > item {
                        dst + (item - src)
                    } else {
                        item
                    }
                }
            }
        }
    }
}

impl Task for State {
    type Input<'a> = ParsedLine where Self: 'a;

    type Output<'a> = u32 where Self: 'a;

    fn parse<'a>(&self, line: &'a str) -> Self::Input<'a> {
        ParsedLine::parse(line)
    }

    fn process(&mut self, input: Self::Input<'_>) {
        match input {
            ParsedLine::Empty => {
                self.current_map = None;
            }
            ParsedLine::Seeds(mut seeds) => {
                seeds.sort();
                self.seeds = seeds;
            }
            ParsedLine::Map(map) => {
                self.current_map = Some(map);
            }
            ParsedLine::Values(a, b, c) => {
                let map = self.current_map.unwrap();
                let map = self.maps.entry(map).or_default();
                match map.binary_search(&(a, b, c)) {
                    Ok(_) => panic!("Duplicate map entry: {:?}", (a, b, c)),
                    Err(pos) => map.insert(pos, (a, b, c)),
                }
            }
        }
    }

    fn output(&mut self) -> Self::Output<'_> {
        let mut low = u32::MAX;
        for &seed in &self.seeds {
            let location = self.lookup(seed);
            if location < low {
                low = location;
            }
        }
        low
    }
}

fn main() {
    let mut state = State::default();
    let res = state.run("adv-2023-day5/input/list.txt");
    println!("{}", res);
}
