use std::io::Result;

use adventofcode_2023::input_helper;

enum Cube {
    Blue(u32),
    Green(u32),
    Red(u32)
}

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

    let sum_of_possible_game_ids: u32 = lines.into_iter()
        .map(|line| {
            let game = process_line_part1(&line.unwrap());

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

fn process_line_part1(line: &str) -> Game {
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
    let is_game_possible = is_game_possible(&parse_sets(sets_part), limit);

    Game { id: game_id, is_possible: is_game_possible }
}

fn is_game_possible(sets: &[CubeCount], limit: CubeCount) -> bool {
    let mut max = 
        CubeCount{
            blue: 0,
            green: 0,
            red: 0
        };

    for set in sets {
        if set.blue > max.blue {
            max.blue = set.blue;
        }

        if set.green > max.green {
            max.green = set.green;
        }
        
        if set.red > max.red {
            max.red = set.red;
        }
    }

    if max.blue > limit.blue {
        return false;
    }

    if max.green > limit.green {
        return false;
    }

    if max.red > limit.red {
        return false;
    }

    return true
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
    mod process_line_part1 {
        use crate::Game;
        use crate::process_line_part1;

        #[test]
        fn it_parses_game_id() {
            assert_eq!(
                Game { id: 23, is_possible: true }, 
                process_line_part1("Game 23: "),
                "Game 23"
            );
            assert_eq!(
                Game { id: 42, is_possible: true }, 
                process_line_part1("Game 42: "),
                "Game 42"
            );
        }

        #[test]
        fn it_recognizes_possible_games() {
            assert_eq!(
                Game { id: 1, is_possible: true }, 
                process_line_part1("Game 1: 14 blue, 13 green, 12 red"),
                "possible game with one subset"
            );
            assert_eq!(
                Game { id: 1, is_possible: true }, 
                process_line_part1("Game 1: 14 blue; 13 green; 12 red"),
                "possible game with three subsets"
            );
        }

        #[test]
        fn it_recognizes_impossible_games() {
            assert_eq!(
                Game { id: 1, is_possible: false }, 
                process_line_part1("Game 1: 15 blue"),
                "impossible game with too many blue cubes"
            );
            assert_eq!(
                Game { id: 1, is_possible: false }, 
                process_line_part1("Game 1: 14 green"),
                "impossible game with too many green cubes"
            );
            assert_eq!(
                Game { id: 1, is_possible: false }, 
                process_line_part1("Game 1: 13 red"),
                "impossible game with too many red cubes"
            );
            assert_eq!(
                Game { id: 1, is_possible: false }, 
                process_line_part1("Game 1: 14 blue; 13 green; 13 red"),
                "impossible game with three subsets"
            );
        }
    }
}