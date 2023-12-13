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

    Ok(())
}

fn process(lines: &Vec<String>) -> u32 {
    let mut lines = lines.iter();

    let instructions = lines.next().expect("Instruction line is missing");
    lines.next().expect("Separation line between instructions and network is missing");

    let mut map = HashMap::new();

    lines
        .map(|line| line.parse::<Node>().unwrap() )
        .for_each(|node| { map.insert(node.key.clone(), node); } );

    count_steps_to_reach_zzz(&instructions, &map)
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
        fn it_should_count_steps_needed_through_the_network() {
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
}