use std::io;

use adventofcode_2023::input_helper::read_lines;

fn main() -> io::Result<()> {
    let lines = read_lines("input/day11").unwrap().map(|e| e.unwrap()).collect();
    println!("Part 1: {}", process(lines));

    Ok(())
}

fn process(lines: Vec<String>) -> usize {
    let expanded_universe = expand(&lines);
    let galaxies = location_of_galaxies(&expanded_universe);

    galaxies.iter().enumerate()
        .map(|(index_g1, g1)|

        galaxies.iter().enumerate()
            .filter(|(index_g2, _)| index_g1 < *index_g2)
            .map(|(_, g2)| distance(*g1, *g2) )
            .collect::<Vec<usize>>()
        )
        .flatten()
        .sum::<usize>()
}

fn expand(lines: &Vec<String>) -> Vec<String> {
    let mut expanded_lines = lines.clone();

    // expand rows
    for (index, row) in lines.iter().enumerate().rev() {
        if row.chars().all(|e| e == '.') {
            expanded_lines.insert(index, row.clone());
        }
    }

    // expand columns
    let mut columns_to_expand = Vec::new();
    for (index, _) in lines[0].chars().enumerate() {
        let mut column = 
            lines.iter().map(|e| e.chars().nth(index).unwrap());

        if column.all(|e| e == '.') {
            columns_to_expand.push(index);
        }
    }

    for index in columns_to_expand.iter().rev() {
        for line in expanded_lines.iter_mut() {
            line.insert(*index, '.');
        }
    }

    expanded_lines
}

fn location_of_galaxies(universe: &Vec<String>) -> Vec<(usize, usize)> {
    universe.iter().enumerate()
        .map(
        |(y, line)| {
            line.char_indices()
                .filter(|(_, c) | *c == '#' )
                .map(move |(x, _)| (x, y))
        })
        .flatten()
        .collect()
}

fn distance(g1: (usize, usize), g2: (usize, usize)) -> usize {
    let vertical_distance = if g2.1 > g1.1 { g2.1 - g1.1 } else { g1.1 - g2.1 };
    let horizontal_distance = if g2.0 > g1.0 { g2.0 - g1.0 } else { g1.0 - g2.0 };

    vertical_distance + horizontal_distance
}

#[cfg(test)]
mod test {
    mod process {
        use crate::process;

        const SAMPLE_INPUT_1: &str =
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        #[test]
        fn it_should_sum_up_all_pairwise_shortest_distances_in_expanded_universe() {
            assert_eq!(
                374,
            
                process(
                    SAMPLE_INPUT_1
                        .lines()
                        .map(|e| e.to_string())
                        .collect()));
        }
    }
}