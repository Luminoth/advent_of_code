use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

use rayon::prelude::*;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, strum::EnumString)]
enum MapType {
    #[strum(serialize = "seed-to-soil")]
    SeedToSoil,

    #[strum(serialize = "soil-to-fertilizer")]
    SoilToFertilizer,

    #[strum(serialize = "fertilizer-to-water")]
    FertilizerToWater,

    #[strum(serialize = "water-to-light")]
    WaterToLight,

    #[strum(serialize = "light-to-temperature")]
    LightToTemperature,

    #[strum(serialize = "temperature-to-humidity")]
    TemperatureToHumidity,

    #[strum(serialize = "humidity-to-location")]
    HumidityToLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MapRange {
    src_range: Range<usize>,
    dst_range: Range<usize>,
}

impl From<&str> for MapRange {
    fn from(v: &str) -> Self {
        let parts = v.split_ascii_whitespace().collect::<Vec<_>>();
        let len = parts[2].parse::<usize>().unwrap();

        let dst_start = parts[0].parse::<usize>().unwrap();
        let dst_range = dst_start..(dst_start + len);

        let src_start = parts[1].parse::<usize>().unwrap();
        let src_range = src_start..(src_start + len);

        Self {
            src_range,
            dst_range,
        }
    }
}

#[derive(Debug)]
struct Map {
    r#type: MapType,
    ranges: Vec<MapRange>,
}

impl From<&str> for Map {
    fn from(v: &str) -> Self {
        let re = Regex::new(r"(?<type>.*) map:\n(?<ranges>(.|\n)*)").unwrap();
        let caps = re.captures(v).unwrap();

        let r#type = MapType::from_str(&caps["type"]).unwrap();
        let ranges = caps["ranges"]
            .trim()
            .split('\n')
            .map(MapRange::from)
            .collect::<Vec<_>>();

        Self { r#type, ranges }
    }
}

#[derive(Debug, Default)]
struct Almanac {
    maps: HashMap<MapType, Map>,
}

impl Almanac {
    fn new(maps: HashMap<MapType, Map>) -> Self {
        Self { maps }
    }
}

fn get_seed_location(seed: usize, almanac: &Almanac) -> usize {
    //println!("seed: {}", seed);

    let map = &almanac.maps[&MapType::SeedToSoil];
    let soil = match map.ranges.iter().find(|r| r.src_range.contains(&seed)) {
        Some(r) => r.dst_range.start + (seed - r.src_range.start),
        None => seed,
    };

    //println!("soil: {}", soil);

    let map = &almanac.maps[&MapType::SoilToFertilizer];
    let fertilizer = match map.ranges.iter().find(|r| r.src_range.contains(&soil)) {
        Some(r) => r.dst_range.start + (soil - r.src_range.start),
        None => soil,
    };

    //println!("fertilizer: {}", fertilizer);

    let map = &almanac.maps[&MapType::FertilizerToWater];
    let water = match map
        .ranges
        .iter()
        .find(|r| r.src_range.contains(&fertilizer))
    {
        Some(r) => r.dst_range.start + (fertilizer - r.src_range.start),
        None => fertilizer,
    };

    //println!("water: {}", water);

    let map = &almanac.maps[&MapType::WaterToLight];
    let light = match map.ranges.iter().find(|r| r.src_range.contains(&water)) {
        Some(r) => r.dst_range.start + (water - r.src_range.start),
        None => water,
    };

    //println!("light: {}", light);

    let map = &almanac.maps[&MapType::LightToTemperature];
    let temp = match map.ranges.iter().find(|r| r.src_range.contains(&light)) {
        Some(r) => r.dst_range.start + (light - r.src_range.start),
        None => light,
    };

    //println!("temp: {}", temp);

    let map = &almanac.maps[&MapType::TemperatureToHumidity];
    let humidity = match map.ranges.iter().find(|r| r.src_range.contains(&temp)) {
        Some(r) => r.dst_range.start + (temp - r.src_range.start),
        None => temp,
    };

    //println!("humidity: {}", humidity);

    let map = &almanac.maps[&MapType::HumidityToLocation];
    let location = match map.ranges.iter().find(|r| r.src_range.contains(&humidity)) {
        Some(r) => r.dst_range.start + (humidity - r.src_range.start),
        None => humidity,
    };

    //println!("location: {}", location);

    location
}

fn part1(seeds: &[usize], almanac: &Almanac) {
    let locations = seeds
        .iter()
        .map(|s| get_seed_location(*s, almanac))
        .collect::<Vec<_>>();

    let min_location = locations.iter().min().unwrap();

    assert!(*min_location == 484023871);
    println!("Min Location: {}", min_location);
}

fn part2(seed_ranges: &[Range<usize>], almanac: &Almanac) {
    // this prints 2445858670
    /*println!(
        "part 2 checking {} seeds",
        seed_ranges.iter().map(|x| x.len()).sum::<usize>()
    );*/

    // lol
    let locations = seed_ranges
        .par_iter()
        .flat_map(|r| {
            let r = r.clone();
            r.into_par_iter()
                .map(|s| get_seed_location(s, almanac))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let min_location = locations.iter().min().unwrap();

    assert!(*min_location == 46294175);
    println!("Min Location: {}", min_location);
}

fn main() {
    let input = include_str!("../input.txt");

    let re = Regex::new(r"seeds: (?<seeds>.*)\n\n(?<maps>(.|\n)*)").unwrap();
    let caps = re.captures(input).unwrap();

    let seeds = caps["seeds"]
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let maps = caps["maps"]
        .split("\n\n")
        .map(Map::from)
        .map(|m| (m.r#type, m))
        .collect::<HashMap<_, _>>();

    let almanac = Almanac::new(maps);

    part1(&seeds, &almanac);

    let seed_ranges = seeds
        .chunks(2)
        .map(|r| r[0]..(r[0] + r[1]))
        .collect::<Vec<_>>();

    part2(&seed_ranges, &almanac);
}
