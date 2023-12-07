use std::{
    io::{self, BufRead},
    ops::Range,
};

#[derive(Debug)]
struct Mapping {
    src: Range<i64>,
    dst: i64,
}

impl Mapping {
    fn from_string(s: String) -> Mapping {
        let mut numbers = s
            .split_ascii_whitespace()
            .map(|s| i64::from_str_radix(s, 10).unwrap());
        let dst_start = numbers.next().unwrap();
        let src_start = numbers.next().unwrap();
        let range_length = numbers.next().unwrap();
        assert!(numbers.next().is_none());
        Mapping {
            src: src_start..src_start + range_length,
            dst: dst_start,
        }
    }

    fn convert(mappings: &Vec<Mapping>, input: i64) -> i64 {
        mappings
            .iter()
            .filter_map(|mapping| {
                if mapping.src.contains(&input) {
                    return Some(mapping.dst + (input - mapping.src.start));
                } else {
                    return None;
                }
            })
            .next()
            .unwrap_or(input)
    }

    fn from_stdin(it: &mut dyn Iterator<Item = String>, header: &str) -> Vec<Mapping> {
        while it.next().unwrap().as_str().ne(header) { /* skip line */ }
        it.map_while(|ln| {
            if ln.len() > 0 {
                Some(Mapping::from_string(ln))
            } else {
                None
            }
        })
        .collect()
    }
}

enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

impl Almanac {
    fn from_stdin() -> Almanac {
        let mut lines = io::stdin().lock().lines().filter_map(Result::ok);
        let seeds: Vec<i64> = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .filter_map(|w| i64::from_str_radix(w, 10).ok())
            .collect();

        Almanac {
            seeds: seeds,
            seed_to_soil: Mapping::from_stdin(&mut lines, "seed-to-soil map:"),
            soil_to_fertilizer: Mapping::from_stdin(&mut lines, "soil-to-fertilizer map:"),
            fertilizer_to_water: Mapping::from_stdin(&mut lines, "fertilizer-to-water map:"),
            water_to_light: Mapping::from_stdin(&mut lines, "water-to-light map:"),
            light_to_temperature: Mapping::from_stdin(&mut lines, "light-to-temperature map:"),
            temperature_to_humidity: Mapping::from_stdin(
                &mut lines,
                "temperature-to-humidity map:",
            ),
            humidity_to_location: Mapping::from_stdin(&mut lines, "humidity-to-location map:"),
        }
    }

    fn seed_to_location(&self, seed: i64) -> i64 {
        let soil = Mapping::convert(&self.seed_to_soil, seed);
        let fertilizer = Mapping::convert(&self.soil_to_fertilizer, soil);
        let water = Mapping::convert(&self.fertilizer_to_water, fertilizer);
        let light = Mapping::convert(&self.water_to_light, water);
        let temperature = Mapping::convert(&self.light_to_temperature, light);
        let humidity = Mapping::convert(&self.temperature_to_humidity, temperature);
        let location = Mapping::convert(&self.humidity_to_location, humidity);
        return location;
    }
}

fn main() {
    let almanac = Almanac::from_stdin();
    let lowest_location = almanac
        .seeds
        .iter()
        .map(|&seed| almanac.seed_to_location(seed))
        .min()
        .unwrap();
    println!("Lowest location: {}", lowest_location);

    // part 2 brute force
    let loewst_location_pt2 = almanac
        .seeds
        .chunks(2)
        .map(|x| x[0]..(x[0] + x[1]))
        .flat_map(|range| range.into_iter())
        .map(|seed| almanac.seed_to_location(seed))
        .min()
        .unwrap();
    println!("Lowest location pt2: {:?}", loewst_location_pt2);
}
