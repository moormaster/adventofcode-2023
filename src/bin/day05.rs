use std::{cmp, io, ops};

use adventofcode_2023::input_helper::read_lines;

#[derive(Debug)]
struct MapChain {
    maps: Vec<Map>
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>
}

#[derive(Clone)]
#[derive(Debug)]
struct MapRange {
    source: u32,
    destination: u32,
    length: u32
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
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
            .into_iter()
            .map( |seed| seed_to_location_map.index(seed) )
            .min()
            .unwrap());

    println!(
        "Part 2: {}",
        find_candidate_seed_range_for_minimal_location(&seed_to_location_map)
            .iter()
            .map(
                |seed_range| 
                
                seed_to_location_map.index(*seed_range.start()))
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

    fn reverse(&self, range: Range) -> Vec<Range> {
        let mut source_ranges = vec![range];

        self.maps
            .iter()
            .rev()
            .for_each(
                |map| 
                
                {
                    source_ranges = source_ranges
                        .iter()
                        .map( |range| map.reverse(range) )
                        .flatten()
                        .collect();
                }
            );

        source_ranges
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

    fn reverse(&self, destination: &Range) -> Vec<Range> {
        let mut decomposed_source_ranges = vec![destination.clone()];
        
        let mut ranges = self.ranges.clone();
        ranges.sort_by_key( |range| range.destination );

        ranges
            .iter()
            .filter( |range| range.get_destination_range().intersects_with(destination) )
            .for_each(
                    |range|

                    {
                        let index = decomposed_source_ranges.len()-1;
                        let decomposition = decomposed_source_ranges[index].decomposition(&range.get_destination_range());
                        decomposed_source_ranges.remove(index);

                        for decomposed_range in decomposition.iter() {
                            if range.get_destination_range().contains_range(decomposed_range) {
                                // reverse decomposed parts that match with destination
                                decomposed_source_ranges.push(range.reverse(decomposed_range));
                            } else {
                                // keep other decomposed parts as is
                                decomposed_source_ranges.push(decomposed_range.clone());
                            }
                        }
                    }
                );

        decomposed_source_ranges
    }
}

impl MapRange {
    fn index(&self, index: u32) -> u32 {
        
        if !self.get_source_range().contains(index) {
            panic!("Index {} out of range for MapRange", index);
        }

        self.destination + (index - self.source)
    }

    fn reverse(&self, range: &Range) -> Range {
        assert!(self.get_destination_range().contains_range(range));

        // | ... source ... |
        //         |
        //         v
        //   | ... dest   ... |
        //
        //    | range |
        //    |
        //    v
        //  | src-r |


        let intersected_destination = self.get_destination_range().intersection(range).unwrap();
        let range_offset = intersected_destination.start - self.destination;

        Range { start: self.source + range_offset, length: intersected_destination.length }
    }

    fn get_source_range(&self) -> Range {
        Range { start: self.source, length: self.length }
    }

    fn get_destination_range(&self) -> Range {
        Range { start: self.destination, length: self.length }
    }
}

impl Range {
    fn contains(&self, value: u32) -> bool {
        value >= self.start && value - self.start < self.length
    }

    fn contains_range(&self, other: &Range) -> bool {
        other.start >= self.start
        && other.length <= self.length - (other.start - self.start)
    }

    fn decomposition(&self, other: &Range) -> Vec<Range> {
        let mut decomposition = self.difference(other);
        if let Some(intersection) = self.intersection(other) {
            decomposition.push(intersection);
        }
        decomposition.sort_by_key(|range| range.start);

        decomposition
    }

    fn difference(&self, to_substract: &Range) -> Vec<Range> {
        if !self.intersects_with(to_substract) {
            return vec![self.clone()];
        }
        
        let mut remaining_length = self.length;
        let mut difference = vec![];

        if to_substract.start > self.start {
            remaining_length = remaining_length - (to_substract.start - self.start);

            difference.push(
                Range { 
                    start:  self.start,
                    length: to_substract.start - self.start
                });
        }

        let intersection = self.intersection(to_substract).unwrap();

        let length = remaining_length - intersection.length;

        if length > 0 {
            let start = intersection.start + intersection.length;

            difference.push(
                Range {
                    start,
                    length
                });
        }

        difference
    }

    fn intersection(&self, other: &Range) -> Option<Range> {
        if !self.intersects_with(other) {
            return None;
        }

        let start = 
            if other.start > self.start {
                other.start
            } else {
                self.start
            };

        let length = 
            if other.start > self.start {
                cmp::min(
                    self.length - (other.start - self.start),
                    other.length)
            } else {
                cmp::min(
                    other.length - (self.start - other.start),
                    self.length)
            };
        
        Some(Range { start, length })
    }

    fn intersects_with(&self, other: &Range) -> bool {
        self.contains(other.start) || other.contains(self.start)
    }

    fn to_range(self) -> ops::RangeInclusive<u32> {
        if self.start > 0 {
            self.start..=self.start-1+self.length
        } else {
            self.start..=self.start+self.length-1
        }
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

fn find_candidate_seed_range_for_minimal_location(seed_to_location_map: &MapChain) -> Vec<ops::RangeInclusive<u32>> {
    seed_to_location_map
        .maps
        .last().unwrap() // get humidity-to-location-map
        .ranges.iter()
        .map(|maprange| maprange.get_destination_range())
        .map(|location_range| seed_to_location_map.reverse(location_range) )
        .flatten()
        .map(|range| range.to_range())
        .collect()
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

        const SAMPLE_INPUT: &str = 
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
56 93 4";

        #[test]
        fn it_should_map_sample_seed_13_to_location_35() {
            let sample_input_lines: Vec<String> = 
                SAMPLE_INPUT
                .split("\n")
                .map( |str| str.to_string() )
                .collect();

            let (_, seed_to_location_map) = process(sample_input_lines);
            assert_eq!(
                35,
                seed_to_location_map.index(13),
                "sample input"
            )
        }
    }

    mod map_reverse {
        use crate::{Map, MapRange, Range};

        #[test]
        fn it_should_reverse_a_destination_range_through_a_single_map_range() {
            let map = Map {
                ranges: vec![
                    MapRange { source: 0, destination: 3, length: 1 }
                ]
            };

            assert_eq!(
                vec![ Range{ start: 0, length: 1} ],

                map.reverse(&Range { start: 3, length: 1 }),
                "reverses a range that equals to the destination range of a defined maprange"
            );

            assert_eq!(
                vec![ Range{ start: 0, length: 2} ],

                map.reverse(&Range { start: 0, length: 2 }),
                "keeps a range constant that does not overlap with any defined ranges in the map"
            );

            assert_eq!(
                vec![ Range{ start: 2, length: 1}, Range { start: 0, length: 1 } ],

                map.reverse(&Range { start: 2, length: 2 }),
                "splits the destination range and reverses the overlapping part"
            );
        }

        #[test]
        fn it_should_reverse_a_destination_range_through_a_multiple_map_ranges() {
            let map = Map {
                ranges: vec![
                    MapRange { source: 0, destination: 12, length: 1 },
                    MapRange { source: 5, destination: 15, length: 2 }
                ]
            };

            assert_eq!(
                vec![ 
                    Range { start: 10, length: 2 },
                    Range { start: 0, length: 1 },
                    Range { start: 13, length: 2 },
                    Range { start: 5, length: 2 },
                    Range { start: 17, length: 3 } ],

                map.reverse(&Range { start: 10, length: 10 }),
                "splits the destination range and reverses the overlapping parts"
            );
        }
    }
}