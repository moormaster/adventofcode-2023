use std::{io, collections::HashSet};

use adventofcode_2023::input_helper::read_lines;

fn main() -> io::Result<()> {
    let map: Vec<String> = read_lines("input/day10")?.map(|e| e.unwrap()).collect();

    let farthest_distance = process(&map);

    println!("Part 1: {}", farthest_distance);

    Ok(())
}

fn process(map: &Vec<String>) -> usize {
    let pipe_positions = determine_pipe_tile_positions(map);

    pipe_positions.len() / 2
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
    let next_direction = follow_tile_from_direction(tile.came_from.as_ref().unwrap(), tile.tile);
    let next_position = traverse_position(&tile.position, &next_direction);
    Tile {
        tile: get_tile(map, next_position),
        came_from: Some( next_direction.invert() ),
        position: next_position
    }
}

fn get_tile(map: &Vec<String>, position: (usize, usize)) -> char {
    map[position.1].chars().nth(position.0).unwrap()
}

fn traverse_position(position: &(usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::East => (position.0 + 1, position.1),
        Direction::West => (position.0 - 1, position.1),
        Direction::South => (position.0, position.1 + 1),
        Direction::North => (position.0, position.1 - 1)
    }
}

fn follow_tile_from_direction(from: &Direction, tile: char) -> Direction {
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
    }
}