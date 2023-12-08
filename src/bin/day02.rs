use std::io::Result;

use adventofcode_2023::input_helper;

enum Cube {
    Blue(u32),
    Green(u32),
    Red(u32)
}

#[derive(Debug)]
#[derive(PartialEq)]
struct CubeCount {
    blue: u32,
    green: u32,
    red: u32
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Game {
    id: u32,
    is_possible: bool
}

fn main() -> Result<()> {
    let lines = input_helper::read_lines("input/day02")?;
    let games: Vec<Game> = lines.into_iter()
        .map(|line| {
            process_line(&line.unwrap())
        })
        .collect();

    let sum_of_possible_game_ids: u32 = games.iter()
        .map(|game| {
            if game.is_possible {
                game.id
            }
            else {
                0
            }
        })
        .sum();

    println!("part1: {sum_of_possible_game_ids}");

    Ok(())
}

fn process_line(line: &str) -> Game {
    let limit = 
        CubeCount { 
            blue: 14,
            green: 13,
            red: 12
        };

    let mut game_and_sets = line.split(": ");
    let game_part = game_and_sets.next().expect("line is missing separator ': '");
    let sets_part = game_and_sets.next().expect("line is missing separator ': '");

    let game_id = parse_game_id(game_part);
    let sets = parse_sets(sets_part);
    let is_game_possible = is_game_possible(&sets, limit);

    Game { id: game_id, is_possible: is_game_possible }
}

fn is_game_possible(sets: &[CubeCount], limit: CubeCount) -> bool {
    let minimal_required_set = get_minimal_required_set(&sets);

    if minimal_required_set.blue > limit.blue {
        return false;
    }

    if minimal_required_set.green > limit.green {
        return false;
    }

    if minimal_required_set.red > limit.red {
        return false;
    }

    return true
}

fn get_minimal_required_set(sets: &[CubeCount]) -> CubeCount {
    let minimial_required_cubecounts = 
        aggregate_cube_counts(
            sets, 
            |is_first_element, aggregated_value, value| {
                if is_first_element {
                    value
                } else if aggregated_value < value {
                    value
                } else {
                    aggregated_value
                }
            });

    minimial_required_cubecounts
}

fn aggregate_cube_counts(sets: &[CubeCount], aggregate: impl Fn(bool, u32, u32) -> u32) -> CubeCount {
    let mut is_first_value = true;

    let mut aggregated_cube = 
        CubeCount{
            blue: 0,
            green: 0,
            red: 0
        };

    for set in sets {
        aggregated_cube.blue = aggregate(is_first_value, aggregated_cube.blue, set.blue);
        aggregated_cube.green = aggregate(is_first_value, aggregated_cube.green, set.green);
        aggregated_cube.red = aggregate(is_first_value, aggregated_cube.red, set.red);

        is_first_value = false;
    }

    return aggregated_cube;
}

fn parse_game_id(game_part: &str) -> u32 {
    game_part["Game ".chars().count()..].parse().expect(&format!("failed to parse game id from game part: {game_part}"))
}

fn parse_sets(sets_part: &str) -> Vec<CubeCount> {
    let sets = sets_part.split("; ");
    let mut result = vec![];

    for set_part in sets {
        let set = parse_set(set_part);
        result.push(set);
    }

    result
}

fn parse_set(set: &str) -> CubeCount {
    let cubes = set.split(", ");
    let mut cube_count = 
        CubeCount {
            blue: 0,
            green: 0,
            red: 0
        };

    for cube_part in cubes {
        if cube_part.len() == 0 {
            continue;
        }

        let cube = parse_cube(cube_part);

        match cube {
            Cube::Blue(count) 
                => cube_count.blue = count,
            Cube::Green(count) 
                => cube_count.green = count,
            Cube::Red(count) 
                => cube_count.red = count
        };
    }

    cube_count
}

fn parse_cube(cube_part: &str) -> Cube {
    let mut count_and_color = cube_part.split(" ");

    let count_part = count_and_color.next().expect(&format!("Cube is missing ' ' char: {cube_part}"));
    let color_part = count_and_color.next().expect(&format!("Cube is missing ' ' char: {cube_part}"));

    let count: u32 = count_part.parse().expect(&format!("failed to parse cube count: {cube_part}"));

    match color_part {
        "blue" => Cube::Blue(count),
        "green" => Cube::Green(count),
        "red" => Cube::Red(count),
        _ => panic!("Unknown cube color: {color_part}")
    }
}

#[cfg(test)]
mod test {
    mod get_minimal_required_set {
        use crate::{get_minimal_required_set, CubeCount};

        #[test]
        fn it_determines_the_min_required_cube_counts_per_color() {
            assert_eq!(
                CubeCount { blue: 3, green: 4, red: 5 },
                get_minimal_required_set(&[
                    CubeCount { blue: 3, green: 1, red: 1 },
                    CubeCount { blue: 1, green: 4, red: 1 },
                    CubeCount { blue: 1, green: 1, red: 5 },
                ])
            );
        }
    }

    mod process_line {
        use crate::process_line;

        #[test]
        fn it_parses_game_id() {
            assert_eq!(
                23,
                process_line("Game 23: ").id,
                "Game 23"
            );
            assert_eq!(
                42,
                process_line("Game 42: ").id,
                "Game 42"
            );
        }

        #[test]
        fn it_recognizes_possible_games() {
            assert!(
                process_line("Game 1: 14 blue, 13 green, 12 red").is_possible,
                "possible game with one subset"
            );
            assert!(
                process_line("Game 1: 14 blue; 13 green; 12 red").is_possible,
                "possible game with three subsets"
            );
        }

        #[test]
        fn it_recognizes_impossible_games() {
            assert!(
                !process_line("Game 1: 15 blue").is_possible,
                "impossible game with too many blue cubes"
            );
            assert!(
                !process_line("Game 1: 14 green").is_possible,
                "impossible game with too many green cubes"
            );
            assert!(
                !process_line("Game 1: 13 red").is_possible,
                "impossible game with too many red cubes"
            );
            assert!(
                !process_line("Game 1: 14 blue; 13 green; 13 red").is_possible,
                "impossible game with three subsets"
            );
        }
    }
}