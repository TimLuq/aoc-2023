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
    Seeds(Vec<(u64, u64)>),
    Map(MapType),
    Values(u64, u64, u64),
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
        let mut it = line[7..].trim().split(' ');
        while let Some(a) = it.next() {
            nums.push((
                a.parse::<u64>().unwrap(),
                it.next().unwrap().parse::<u64>().unwrap(),
            ));
        }
        ParsedLine::Seeds(nums)
    }

    fn parse_values(line: &str) -> Self {
        let mut parts = line.split(' ');
        let a = parts.next().unwrap().parse::<u64>().unwrap();
        let b = parts.next().unwrap().parse::<u64>().unwrap();
        let c = parts.next().unwrap().parse::<u64>().unwrap();
        // swap destination and source for better lookup
        ParsedLine::Values(b, a, c)
    }
}

#[derive(Debug, Default)]
struct State {
    current_map: Option<MapType>,
    seeds: Vec<(u64, u64)>,
    maps: BTreeMap<MapType, Vec<(u64, u64, u64)>>,
}

impl State {
    /// lookup from seed to closest location and the length of identical ones
    fn lookup(&self, seed: u64) -> (u64, u64) {
        let (soil, min) = self.lookup_map(MapType::SeedToSoil, seed);
        let (fertilizer, minv) = self.lookup_map(MapType::SoilToFertilizer, soil);
        let min = min.min(minv);
        let (water, minv) = self.lookup_map(MapType::FertilizerToWater, fertilizer);
        let min = min.min(minv);
        let (light, minv) = self.lookup_map(MapType::WaterToLight, water);
        let min = min.min(minv);
        let (temperature, minv) = self.lookup_map(MapType::LightToTemperature, light);
        let min = min.min(minv);
        let (humidity, minv) = self.lookup_map(MapType::TemperatureToHumidity, temperature);
        let min = min.min(minv);
        let (location, minv) = self.lookup_map(MapType::HumidityToLocation, humidity);
        (location, min.min(minv))
    }

    fn lookup_map(&self, map_type: MapType, item: u64) -> (u64, u64) {
        let map = self.maps.get(&map_type).unwrap();
        match map.binary_search_by_key(&item, |(a, _, _)| *a) {
            Ok(pos) => {
                let (_, dst, len) = map[pos];
                (dst, len)
            }
            Err(pos) => {
                if pos == 0 {
                    (item, map[0].0 - item)
                } else {
                    let (src, dst, len) = map[pos - 1];
                    if src + len > item {
                        let offs = item - src;
                        (dst + offs, len - offs)
                    } else if pos < map.len() {
                        (item, map[pos].0 - item)
                    } else {
                        (item, u32::MAX as u64)
                    }
                }
            }
        }
    }
}

impl Task for State {
    type Input<'a> = ParsedLine where Self: 'a;

    type Output<'a> = u64 where Self: 'a;

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
        let mut low = u64::MAX;
        for &(seed, len) in &self.seeds {
            let mut i = 0;
            while i < len {
                let (location, ident_len) = self.lookup(seed + i);
                if location < low {
                    low = location;
                }
                i += ident_len;
            }
        }
        low
    }
}

fn main() {
    let mut state = State::default();
    let time = std::time::Instant::now();
    let res = state.run("adv-2023-day5/input/list.txt");
    let elapsed = time.elapsed();
    println!("Result: {}", res);
    println!("Elapsed: {}us", elapsed.as_micros());
}
