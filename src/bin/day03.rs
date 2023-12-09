use std::{io::Result, ops::Range};

use adventofcode_2023::input_helper;

#[derive(Debug)]
#[derive(PartialEq)]
struct ParsedValue {
    value: u32,
    range: Range<usize>
}

fn main() -> Result<()>{
    let mut lines = input_helper::read_lines("input/day03").unwrap();

    let first_line = lines.next().unwrap().unwrap();
    let second_line = lines.next().unwrap().unwrap();

    let mut part_numbers = vec![];

    let mut line_above: Option<String> = None;
    let mut line = first_line;
    let mut line_below = Some(second_line);

    part_numbers.append(&mut get_part_numbers(None, &line, line_below.as_deref()));


    for line_read in lines {
        {
            if let Some(line_below) = line_below {
                    line_above = Some(line);
                    line = line_below;
            }
        }
        line_below = Some(line_read.unwrap());
        
        part_numbers.append(&mut get_part_numbers(line_above.as_deref(), &line, line_below.as_deref()));
    }

    {
        if let Some(line_below) = line_below {
                line_above = Some(line);
                line = line_below;
        }
    }
    part_numbers.append(&mut get_part_numbers(line_above.as_deref(), &line, None));

    println!("Part 1: {}", part_numbers.into_iter().sum::<u32>());

    Ok(())
}

fn get_part_numbers(line_above: Option<&str>, line: &str, line_below: Option<&str>) -> Vec<u32> {
    let line_above_len: usize = 
        if let Some(line_above) = line_above {
            line_above.len()
        } else {
            0
        };
    let line_len: usize = line.len();
    let line_below_len: usize = 
        if let Some(line_below) = line_below {
            line_below.len()
        } else {
            0
        };

    let mut values = vec![];
    let parsed_numbers = parse_numbers(&line);

    for parsed_number in parsed_numbers {
        let mut has_adjacent_symbol = false;

        // check for adjacent symbol in line above
        if let Some(line_above) = line_above {
            let surrounding_range = 
                get_surrounding_range(&parsed_number.range, 0..line_above_len);
            
            if line_above.get(surrounding_range).unwrap().chars().any(|char| is_symbol(char)) {
                has_adjacent_symbol = true;
            }
        };

        // check for adjacent symbols in current line
        let surrounding_range = 
            get_surrounding_range(&parsed_number.range, 0..line_len);
        let chars_iterator = line.chars();
        let mut chars_iterator = chars_iterator.skip(surrounding_range.start);
        
        if parsed_number.range.start > 0 {
            if let Some(c) = chars_iterator.next() {
                if is_symbol(c) {
                    has_adjacent_symbol = true;
                }
            }
        }

        let mut chars_iterator = chars_iterator.skip(parsed_number.range.len());

        if let Some(c) = chars_iterator.next() {
            if is_symbol(c) {
                has_adjacent_symbol = true;
            }
        }

        // check for adjacent symbol in line below
        if let Some(line_below) = line_below {
            let surrounding_range = 
                get_surrounding_range(&parsed_number.range, 0..line_below_len);
            
                if line_below.get(surrounding_range).unwrap().chars().any(|char| is_symbol(char)) {
                    has_adjacent_symbol = true;
                }
        };

        if has_adjacent_symbol {
            values.push(parsed_number.value);
        }
    }

    values
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn get_surrounding_range(range: &Range<usize>, bounds: Range<usize>) -> Range<usize> {
    Range { 
        start: if range.start > bounds.start { range.start-1 } else { bounds.start },
        end: if range.end < bounds.end { range.end+1 } else { bounds.end }
    }
}

fn parse_numbers(line: &str) -> Vec<ParsedValue> {
    let mut numbers = vec![];

    let mut char_indices_iterator = line.char_indices();

    while let Some(mut index_and_character) = char_indices_iterator.next() {
        let mut number_range: Range<usize> = index_and_character.0..index_and_character.0;

        while index_and_character.1.is_digit(10) {
            number_range.end = index_and_character.0 + 1;

            match char_indices_iterator.next() {
                Some(c) => {
                    index_and_character = c;
                }
                None => { break }
            }
        }

        if let Ok(value) = line[number_range.clone()].parse() {
            numbers.push(ParsedValue {
                value: value,
                range: number_range
            });
        }
    }

    numbers
}

#[cfg(test)]
mod test
{
    mod parse_numbers {
        use crate::{parse_numbers, ParsedValue};

        #[test]
        fn it_parses_all_numbers_from_a_line() {
            assert_eq!(
                vec![ParsedValue { value: 42, range: 0..2}], 
                parse_numbers("42"), 
                "single number"
            );

            assert_eq!(
                vec![ParsedValue { value: 42, range: 3..5}], 
                parse_numbers("..$42*.."), 
                "single number enclosed with symbols or dots"
            );

            assert_eq!(
                vec![
                    ParsedValue { value: 1, range: 0..1 },
                    ParsedValue { value: 23, range: 4..6 },
                    ParsedValue { value: 42, range: 9..11 }
                    ], 
                parse_numbers("1..$23*..42"), 
                "multiple numbers enclosed with symbols or dots"
            );
        }
    }

    mod get_part_numbers {
        use crate::get_part_numbers;

        #[test]
        fn it_returns_numbers_adjacent_to_a_symbol() {
            assert_eq!(
                vec![0u32; 0],
                get_part_numbers(
                    None,
                    "2",
                    None
                ),
                "single non-part number"
            );

            assert_eq!(
                vec![2],
                get_part_numbers(
                    None,
                    "*2",
                    None
                ),
                "single part number"
            );

            assert_eq!(
                vec![2, 3],
                get_part_numbers(
                    None,
                    "1.2*3.4",
                    None
                ),
                "single line with multiple numbers"
            );

            assert_eq!(
                vec![1, 3],
                get_part_numbers(
                    Some(   "......*"),
                            ".1.2.3.",
                    Some(   "$......")
                ),
                "multiple lines with some part numbers"
            );
        }
    }
}

