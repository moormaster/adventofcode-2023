use std::{io, str::FromStr, collections::HashMap};

use adventofcode_2023::input_helper::read_lines;

fn main() -> io::Result<()> {
    let lines: Vec<String> = 
        read_lines("input/day08").unwrap()
        .into_iter()
        .map(|line| line.unwrap() )
        .collect();

    println!(
        "Part 1: {}",
        process(&lines)
    );

    println!(
        "Part 2: {}",
        process_part2(&lines)
    );

    Ok(())
}

fn process(lines: &Vec<String>) -> u32 {
    let (instructions, map) = parse_instructions_and_map(lines);

    count_steps_to_reach_zzz(&instructions, &map)
}

fn process_part2(lines: &Vec<String>) -> usize {
    let (instructions, map) = parse_instructions_and_map(lines);

    count_steps_to_simultaneously_reach_nodes_ending_with_z(&instructions, &map)
}

fn parse_instructions_and_map(lines: &Vec<String>) -> (String, HashMap<String, Node>) {
    let mut lines = lines.iter();

    let instructions = lines.next().cloned().expect("Instruction line is missing");
    lines.next().expect("Separation line between instructions and network is missing");

    let mut map = HashMap::new();

    lines
        .map(|line| line.parse::<Node>().unwrap() )
        .for_each(|node| { map.insert(node.key.clone(), node); } );

    (instructions, map)
}

fn count_steps_to_reach_zzz(instructions: &str, map: &HashMap<String, Node>) -> u32 {
    const START_KEY: &str = "AAA";
    const TARGET_KEY: &str = "ZZZ";

    let mut steps = 0;
    let mut node = map.get(START_KEY).expect(&format!("Start node '{}' not found", START_KEY));

    while node.key != TARGET_KEY {
        instructions
            .chars().into_iter()
            .for_each(
                |direction| {
                    match direction {
                        'L' => {
                            node = 
                                map.get(&node.left)
                                    .expect(&format!("Node '{}' specified a left key '{}' for a node that does not exist!", node.key, node.left));

                                steps += 1;
                        }

                        'R' => {
                            node = 
                                map.get(&node.right)
                                    .expect(&format!("Node '{}' specified a right key '{}' for a node that does not exist!", node.key, node.right));

                            steps += 1;
                        }

                        _ => { panic!("Invalid direction: '{}'", direction); }
                    }
                }
            );
    }

    steps
}

fn count_steps_to_simultaneously_reach_nodes_ending_with_z(instructions: &str, map: &HashMap<String, Node>) -> usize {
    let mut steps = 0;

    let mut node_cycles: Vec<NodeCycle> =
        map
            .iter()
            .filter(|pair| pair.1.key.ends_with("A"))
            .map(
                |pair| 
                
                NodeCycle {
                    current_node: &map[&pair.1.key],
                    complete: false,
                    z_seen: false,
                    cycle: 
                        Cycle {
                            pos: 0,
                            size: 0
                        }
                })
            .collect();

    assert!(node_cycles.len() > 0, "Did not find any start nodes (ending on A)");

    // for each "current node" we want to determine
    // - how big is the cycle between two occurrences of "..Z" nodes? (size)
    // - how far the next "..Z" node away? (offset)
    while node_cycles.iter().any(|node_cycle| !node_cycle.complete) {
        instructions
            .chars().into_iter()
            .for_each(
                |direction| {
                    node_cycles
                        .iter_mut()
                        .for_each(
                            |node_cycle| {
                                match direction {
                                    'L' => {
                                        node_cycle.current_node = 
                                            map.get(&node_cycle.current_node.left)
                                                .expect(&format!("Node '{}' specified a left key '{}' for a node that does not exist!", node_cycle.current_node.key, node_cycle.current_node.left))
                                    }

                                    'R' => {
                                        node_cycle.current_node = 
                                            map.get(&node_cycle.current_node.right)
                                                .expect(&format!("Node '{}' specified a right key '{}' for a node that does not exist!", node_cycle.current_node.key, node_cycle.current_node.right))
                                    }

                                    _ => { panic!("Invalid direction: '{}'", direction); }
                                }

                                node_cycle.cycle.pos += 1;
                                if node_cycle.z_seen && !node_cycle.complete {
                                    node_cycle.cycle.size += 1;
                                }

                                if node_cycle.current_node.key.ends_with("Z") {
                                    if !node_cycle.z_seen {
                                        node_cycle.z_seen = true;
                                    } else if !node_cycle.complete {
                                        node_cycle.complete = true;
                                    }
                                }
                            }
                        );

                    steps += 1;
                }
            );
    }

    count_steps_until_all_cycles_are_complete(
        node_cycles.iter()
                    .map(
                        |node_cycle|

                        Cycle {
                            pos: node_cycle.cycle.pos,
                            size: node_cycle.cycle.size                 
                        })
                    .collect()
    )
}

fn count_steps_until_all_cycles_are_complete(mut cycles: Vec<Cycle>) -> usize {
    let gcd_of_cycle_sizes = 
        cycles.iter().fold(cycles[0].size, |acc, e| { gcd(acc, e.size) });

    // advancing all cycles until first cycle is aligned
    while cycles[0].pos % cycles[0].size > 0 {
        cycles.iter_mut().for_each(|e| { e.pos = e.pos + 1 });
    }

    // advancing all cycles until all cycles are aligned aligned
    while cycles.iter().any( |e| e.pos % e.size > 0 ) {
        cycles.iter_mut().for_each(|e| { e.pos = e.pos + gcd_of_cycle_sizes });
    }
    
    cycles.last().unwrap().pos
}

#[derive(Debug)]
struct NodeCycle<'a> {
    current_node: &'a Node,
    z_seen: bool,
    complete: bool,
    cycle: Cycle
}

#[derive(Debug)]
struct Cycle {
    pos: usize,
    size: usize
}

#[derive(Debug)]
struct Node {
    key: String,
    left: String,
    right: String
}

impl FromStr for Node {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key, links) = s.split_once(" = ").expect(&format!("missing delimiter ' = ': '{}'", s));

        assert!(links.starts_with("("), );
        assert!(links.ends_with(")"), "missing ')' at the end: '{}'", s);

        let links = 
            links
                .strip_prefix("(").expect(&format!("missing '(' after ' = ': '{}'", s))
                .strip_suffix(")").expect(&format!("missing ')' at the end of link leys ' = ': '{}'", s));

        let (left, right) = links.split_once(", ").expect("missing delimiter ', ' between left and right link keys");

        Ok(
            Node {
                key: key.to_string(),
                left: left.to_string(),
                right: right.to_string()
            }
        )
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let b_tmp = b;
        b = a % b;
        a = b_tmp;
    }

    a
}

#[cfg(test)]
mod test {
    mod process {
        use crate::process;

        const SAMPLE_INPUT_2_STEPS: &str =
"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        const SAMPLE_INPUT_6_STEPS: &str =
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        #[test]
        fn it_should_count_steps_needed_to_traverse_through_the_network() {
            assert_eq!(
                2,

                process(&SAMPLE_INPUT_2_STEPS.split("\n").map(|e| e.to_string()).collect()),
                "instructions for network that takes 2 steps"
            );

            assert_eq!(
                6,

                process(&SAMPLE_INPUT_6_STEPS.split("\n").map(|e| e.to_string()).collect()),
                "instructions for network that takes 6 steps"
            );
        }
    }

    mod process_part2 {
        use crate::process_part2;

        const SAMPLE_INPUT_6_STEPS: &str =
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        #[test]
        fn it_should_count_steps_needed_to_simultaneously_traverse_through_the_network() {
            assert_eq!(
                6,

                process_part2(&SAMPLE_INPUT_6_STEPS.split("\n").map(|e| e.to_string()).collect()),
                "instructions for network that takes 6 steps"
            );
        }
    }

    mod gcd {
        use crate::gcd;

        #[test]
        fn it_should_calculate_the_greatest_common_divisor_for_primes() {
            assert_eq!(1, gcd(3, 5));
            assert_eq!(1, gcd(5, 7));
            assert_eq!(1, gcd(3, 11));
        }

        #[test]
        fn it_should_calculate_the_greatest_common_divisor_for_numbers_with_common_prime_factors() {
            assert_eq!(5, gcd(3*5, 5*7));
            assert_eq!(3*3, gcd(2*3*3*5, 3*3*7));
        }
    }
}