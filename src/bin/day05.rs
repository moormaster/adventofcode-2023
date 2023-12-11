use std::io;

use adventofcode_2023::input_helper::read_lines;

#[derive(Debug)]
struct MapChain {
    maps: Vec<Map>
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>
}

#[derive(Debug)]
struct MapRange {
    source: u32,
    destination: u32,
    length: u32
}

#[derive(Debug)]
struct Range {
    start: u32,
    length: u32
}

fn main() -> io::Result<()> {
    let lines: Vec<String> = 
        read_lines("input/day05")
        .unwrap()
        .map( |result| result.ok().unwrap() )
        .collect();

    let (seeds, seed_to_location_map) = process(lines);

    println!(
        "Part 1: {}",
        seeds
            .iter()
            .map( |seed| seed_to_location_map.index(*seed) )
            .min()
            .unwrap());

    Ok(())
}

impl MapChain {
    fn index(&self, index: u32) -> u32 {
        let mut value = index;

        self.maps
            .iter()
            .map( |map| { value = map.index(value); value } )
            .last()
            .unwrap_or(value)
    }
}

impl Map {
    fn index(&self, index: u32) -> u32 {
        self.ranges
            .iter()
            .filter(
                |map_range| map_range.get_source_range().contains(index)
            )
            .next()
            .and_then( |map_range| Some(map_range.index(index)) )
            .or(Some(index))
            .unwrap()
    }
}

impl MapRange {
    fn index(&self, index: u32) -> u32 {
        
        if !self.get_source_range().contains(index) {
            panic!("Index {} out of range for MapRange", index);
        }

        self.destination + (index - self.source)
    }

    fn get_source_range(&self) -> Range {
        Range { start: self.source, length: self.length }
    }
}

impl Range {
    fn contains(&self, value: u32) -> bool {
        value >= self.start && value - self.start < self.length
    }
}

fn process(lines: Vec<String>) -> (Vec<u32>, MapChain) {
    let mut mapchain = 
        MapChain {
            maps: vec![]
        };

    // parse seeds
    let mut lines = lines.into_iter();
    let seeds = lines.next().unwrap();
    let (_, seeds) = seeds.split_once(":").unwrap();
    let seeds = 
        seeds
            .split_whitespace()
            .map( |seed| seed.parse::<u32>().unwrap() )
            .collect();
    let empty_line = lines.next().unwrap();
    if empty_line != "" {
        panic!("Seed list not followed by an empty line");
    }

    // parse maps
    while let Some(map) = process_map(&mut lines) {
        mapchain.maps.push(map);
    }

    (seeds, mapchain)
}

fn process_map(lines: &mut dyn Iterator<Item = String>) -> Option<Map> {
    if let None = lines.next() {
        return None;
    };

    let mut map =
        Map {
            ranges: vec![]
        };

    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }

        let mut items = line.split(" ");
        let destination = items.next().expect(&format!("destination not present in range: {}", line));
        let source = items.next().expect(&format!("source not present in range: {}", line));
        let length = items.next().expect(&format!("length not present in range: {}", line));

        let destination = destination.parse::<u32>().unwrap();
        let source = source.parse::<u32>().unwrap();
        let length = length.parse::<u32>().unwrap();

        map.ranges.push(
            MapRange { 
                source, 
                destination, 
                length
            }
        );
    }

    map.ranges.sort_by_key(|range| range.source);

    if map.ranges.len() > 0 {
        Some(map)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    mod process {
        use crate::process;

        #[test]
        fn it_should_map_sample_seed_13_to_location_35() {
            let sample_input: Vec<String> = 
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
                .split("\n")
                .map( |str| str.to_string() )
                .collect();

            let (_, seed_to_location_map) = process(sample_input);
            assert_eq!(
                35,
                seed_to_location_map.index(13),
                "sample input"
            )
        }
    }
}