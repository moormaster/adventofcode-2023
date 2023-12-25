use std::{io, collections::HashSet};

use adventofcode_2023::input_helper::read_lines;

fn main() -> io::Result<()> {
    let map: Vec<String> = read_lines("input/day10")?.map(|e| e.unwrap()).collect();

    let farthest_distance = process(&map);
    println!("Part 1: {}", farthest_distance);

    let number_of_ground_tiles_enclosed_by_pipe = process_part2(&map);
    println!("Part 2: {}", number_of_ground_tiles_enclosed_by_pipe);

    Ok(())
}

fn process(map: &Vec<String>) -> usize {
    let pipe_positions = determine_pipe_tile_positions(map);

    pipe_positions.len() / 2
}

fn process_part2(map: &Vec<String>) -> usize {
    let pipe_positions = determine_pipe_tile_positions(map);

    let mut number_of_ground_tiles_enclosed_by_pipe = 0;
    for y in 0..map.len() {
        let mut is_inside_pipe_area = false;
        let mut border_begin_tile = None;

        for x in 0..map[0].len() {
            let mut current_tile = get_tile(map, (x, y));
            if current_tile == 'S' {
                current_tile = guess_tile(map, (x, y));
            }

            if pipe_positions.contains(&(x, y)) {
                match (border_begin_tile, current_tile) {
                    (_, 'F') => { border_begin_tile = Some('F'); }
                    (_, 'L') => { border_begin_tile = Some('L'); }
                    (Some('L'), '7')
                    | (Some('F'), 'J') => { 
                        border_begin_tile = None;
                        is_inside_pipe_area = !is_inside_pipe_area;
                    }
                    (Some('F'), '7')
                    | (Some('L'), 'J') => { border_begin_tile = None; }
                    (_, '-') => {
                        // continue iterating "along" the border pipe tiles
                    }

                    _ => { is_inside_pipe_area = !is_inside_pipe_area; }
                }
            } else if is_inside_pipe_area {
                number_of_ground_tiles_enclosed_by_pipe += 1;
            }
        }
    }
    number_of_ground_tiles_enclosed_by_pipe
}

fn determine_pipe_tile_positions(map: &Vec<String>) -> HashSet<(usize, usize)> {
    let mut pipe_positions: HashSet<(usize, usize)> = HashSet::new();
    let start_position = 
        map.iter().enumerate()
            .map(|(y, line)| {
                line.char_indices()
                    .find(|(_, char)| *char == 'S')
                    .and_then(|(x, _)| Some( (x, y) ) )
            })
            .filter(|e| e.is_some())
            .next().unwrap().unwrap();
    pipe_positions.insert(start_position.clone());

    let mut current_tiles = determine_adjacent_tiles(map, start_position);
    for tile in current_tiles.iter() {
        pipe_positions.insert(tile.position.clone());
    }

    while current_tiles.windows(2).any(|e| e[0].position != e[1].position) {
        current_tiles = 
            current_tiles.iter()
                .map(|tile| follow_tile(map, tile))
                .collect();

        for tile in current_tiles.iter() {
            pipe_positions.insert(tile.position.clone());
        }
    }

    pipe_positions
}

fn determine_adjacent_tiles(map: &Vec<String>, position: (usize, usize)) -> Vec<Tile>{
    let mut adjacent_tiles = Vec::new();

    let width = map[0].len();
    let height = map.len();

    // to the west
    if position.0 > 0 {
        let tile = get_tile(map, (position.0-1, position.1));

        if tile == 'L' || tile == 'F' || tile == '-' {
            adjacent_tiles.push(
                Tile {
                    came_from: Some( Direction::East ),
                    position: (position.0-1, position.1),
                    tile
                });
        }
    }

    // to the east
    if position.0 < width-1 {
        let tile = get_tile(map, (position.0+1, position.1));

        if tile == 'J' || tile == '7' || tile == '-' {
            adjacent_tiles.push(
                Tile {
                    came_from: Some( Direction::West ),
                    position: (position.0+1, position.1),
                    tile
                });
        }
    }

    // to the north
    if position.1 > 0 {
        let tile = get_tile(map, (position.0, position.1-1));

        if tile == '7' || tile == 'F' || tile == '|' {
            adjacent_tiles.push(
                Tile {
                    came_from: Some( Direction::South ),
                    position: (position.0, position.1-1),
                    tile
                });
        }
    }

    // to the south
    if position.1 < height-1 {
        let tile = get_tile(map, (position.0, position.1+1));

        if tile == 'J' || tile == 'L' || tile == '|' {
            adjacent_tiles.push(
                Tile {
                    came_from: Some( Direction::North ),
                    position: (position.0, position.1+1),
                    tile
                });
        }
    }
    
    adjacent_tiles
}

struct Tile {
    tile: char,
    position: (usize, usize),
    came_from: Option<Direction>
}

#[derive(Clone)]
#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South
}

impl Direction {
    fn invert(&self) -> Direction {
        match self {
            Direction::East => Direction::West,
            Direction::West => Direction::East,

            Direction::North => Direction::South,
            Direction::South => Direction::North
        }
    }
}

fn follow_tile(map: &Vec<String>, tile: &Tile) -> Tile {
    let next_direction = follow_tile_from_direction(tile.came_from.clone().unwrap(), tile.tile);
    let next_position = traverse_position(&tile.position, next_direction.clone());
    Tile {
        tile: get_tile(map, next_position),
        came_from: Some( next_direction.invert() ),
        position: next_position
    }
}

fn get_tile(map: &Vec<String>, position: (usize, usize)) -> char {
    map[position.1].chars().nth(position.0).unwrap()
}

fn guess_tile(map: &Vec<String>, position: (usize, usize)) -> char {
    let width = map[0].len();
    let height = map.len();

    let north = if position.1 > 0 { Some(get_tile(map, traverse_position(&position, Direction::North))) } else { None };
    let south = if position.1 < height-1 { Some(get_tile(map, traverse_position(&position, Direction::South))) } else { None };
    let east = if position.0 < width-1 { Some(get_tile(map, traverse_position(&position, Direction::East))) } else { None };
    let west = if position.0 > 0 { Some(get_tile(map, traverse_position(&position, Direction::West))) } else { None };

    match (north, east, south, west) {
        (Some('7' | 'F' | '|'), Some('J' | '7' | '-'), _, _ ) => 'L',
        (Some('7' | 'F' | '|'), _, _, Some('F' | 'L' | '-') ) => 'J',
        (_, Some('J' | '7' | '-'), Some('J' | 'L' | '|'), _) => 'F',
        (_, _, Some('J' | 'L' | '|'), Some('F' | 'L' | '-')) => '7',
        _ => { panic!("Cannot guess tile enclosed by (north, east, south, west): ({}, {}, {}, {})", north.unwrap_or(' '), east.unwrap_or(' '), south.unwrap_or(' '), west.unwrap_or(' ')) }
    }
}

fn traverse_position(position: &(usize, usize), direction: Direction) -> (usize, usize) {
    match direction {
        Direction::East => (position.0 + 1, position.1),
        Direction::West => (position.0 - 1, position.1),
        Direction::South => (position.0, position.1 + 1),
        Direction::North => (position.0, position.1 - 1)
    }
}

fn follow_tile_from_direction(from: Direction, tile: char) -> Direction {
    match (from, tile) {
        (Direction::South, 'F') => Direction::East,
        (Direction::East, 'F')  => Direction::South,
        (Direction::West, '7')  => Direction::South,
        (Direction::South, '7') => Direction::West,
        (Direction::North, 'J') => Direction::West,
        (Direction::West, 'J')  => Direction::North,
        (Direction::North, 'L') => Direction::East,
        (Direction::East, 'L')  => Direction::North,
        (Direction::East, '-') => Direction::West,
        (Direction::West, '-')  => Direction::East,
        (Direction::North, '|') => Direction::South,
        (Direction::South, '|')  => Direction::North,

        (from, tile) => panic!("Cannot follow tile {:?} from direction {:?}", tile, from)
    }
}

#[cfg(test)]
mod test {
    mod process {
        use crate::process;
        use crate::process_part2;

        const SAMPLE_INPUT_1: &str =
".....
.S-7.
.|.|.
.L-J.
.....";

        const SAMPLE_INPUT_2: &str =
"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        const SAMPLE_INPUT_3: &str = 
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        const SAMPLE_INPUT_4: &str =
".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        const SAMPLE_INPUT_5: &str =
"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        #[test]
        fn it_should_calculate_the_distance_to_the_farthest_point() {
            assert_eq!(
                4, 
                
                process(&SAMPLE_INPUT_1
                            .split("\n")
                            .map(|e| e.to_string())
                            .collect()));
            assert_eq!(
                8,

                process(&SAMPLE_INPUT_2
                    .split("\n")
                    .map(|e| e.to_string())
                    .collect()));
        }

        #[test]
        fn it_should_calculate_the_number_of_enclosed_ground_tiles() {
            assert_eq!(
                1, 
                
                process_part2(&SAMPLE_INPUT_1
                            .split("\n")
                            .map(|e| e.to_string())
                            .collect()));
            assert_eq!(
                1,

                process_part2(&SAMPLE_INPUT_2
                    .split("\n")
                    .map(|e| e.to_string())
                    .collect()));

            assert_eq!(
                4,

                process_part2(&SAMPLE_INPUT_3
                    .split("\n")
                    .map(|e| e.to_string())
                    .collect()));

            assert_eq!(
                8,

                process_part2(&SAMPLE_INPUT_4
                    .split("\n")
                    .map(|e| e.to_string())
                    .collect()));

            assert_eq!(
                10,

                process_part2(&SAMPLE_INPUT_5
                    .split("\n")
                    .map(|e| e.to_string())
                    .collect()));
        }
    }
}