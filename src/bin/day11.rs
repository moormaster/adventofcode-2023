use std::io;

use adventofcode_2023::input_helper::read_lines;

fn main() -> io::Result<()> {
    let lines = read_lines("input/day11").unwrap().map(|e| e.unwrap()).collect();
    println!("Part 1: {}", process(&lines, 1));

    Ok(())
}

fn process(lines: &Vec<String>, expand_times: usize) -> usize {
    let galaxies = location_of_galaxies(&lines);

    let rows_to_expand = get_rows_to_expand(lines);
    let columns_to_expand = get_columns_to_expand(lines);

    galaxies.iter().enumerate()
        .map(|(index_g1, g1)|

        galaxies.iter().enumerate()
            .filter(|(index_g2, _)| index_g1 < *index_g2)
            .map(|(_, g2)| distance(*g1, *g2, &rows_to_expand, &columns_to_expand, expand_times) )
            .collect::<Vec<usize>>()
        )
        .flatten()
        .sum::<usize>()
}

fn get_rows_to_expand(lines: &Vec<String>) -> Vec<usize> {
    let mut rows_to_expand = Vec::new();

    // expand rows
    for (index, row) in lines.iter().enumerate().rev() {
        if row.chars().all(|e| e == '.') {
            rows_to_expand.push(index)
        }
    }

    rows_to_expand
}

fn get_columns_to_expand(lines: &Vec<String>) -> Vec<usize> {
    // expand columns
    let mut columns_to_expand = Vec::new();
    for (index, _) in lines[0].chars().enumerate() {
        let mut column = 
            lines.iter().map(|e| e.chars().nth(index).unwrap());

        if column.all(|e| e == '.') {
            columns_to_expand.push(index);
        }
    }

    columns_to_expand
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

fn distance(g1: (usize, usize), g2: (usize, usize), rows_to_expand: &Vec<usize>, columns_to_expand: &Vec<usize>, expand_times: usize) -> usize {
    let vertical_distance = if g2.1 > g1.1 { g2.1 - g1.1 } else { g1.1 - g2.1 };
    let horizontal_distance = if g2.0 > g1.0 { g2.0 - g1.0 } else { g1.0 - g2.0 };

    let vertical_expansion: usize =
        rows_to_expand.iter()
            .filter(|row| if g2.1 > g1.1 { **row > g1.1 } else { **row > g2.1 })
            .filter(|row| if g2.1 > g1.1 { **row < g2.1 } else { **row < g1.1 })
            .map(|_| expand_times-1)
            .sum();

    let horizontal_expansion: usize =
        columns_to_expand.iter()
            .filter(|column| if g2.0 > g1.0 { **column > g1.0 } else { **column > g2.0 })
            .filter(|column| if g2.0 > g1.0 { **column < g2.0 } else { **column < g1.0 })
            .map(|_| expand_times-1)
            .sum();

    vertical_distance + vertical_expansion + horizontal_distance + horizontal_expansion
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
                    &SAMPLE_INPUT_1
                        .lines()
                        .map(|e| e.to_string())
                        .collect(),
                        
                    2));
        }
    }
}