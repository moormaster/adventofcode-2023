use std::io as io;
use std::ops::Range;

use adventofcode_2023::input_helper;

#[derive(Debug)]
#[derive(PartialEq)]
struct ParsedItem {
    value: AstItem,
    range: Range<usize>
}

#[derive(Debug)]
#[derive(PartialEq)]
enum AstItem {
    Number { value: u32 },
    Symbol { value: char },
    Dot
}

fn main() -> io::Result<()>{
    let mut lines = input_helper::read_lines("input/day03").unwrap();

    let first_line = lines.next().unwrap().unwrap();
    let second_line = lines.next().unwrap().unwrap();

    let mut part_numbers = vec![];

    let mut line_above: Option<String> = None;
    let mut line = first_line;
    let mut line_below = Some(second_line);

    part_numbers.append(&mut get_part_numbers(None, &line, line_below.as_deref())?);


    for line_read in lines {
        {
            if let Some(line_below) = line_below {
                    line_above = Some(line);
                    line = line_below;
            }
        }
        line_below = Some(line_read.unwrap());
        
        part_numbers.append(&mut get_part_numbers(line_above.as_deref(), &line, line_below.as_deref())?);
    }

    {
        if let Some(line_below) = line_below {
                line_above = Some(line);
                line = line_below;
        }
    }
    part_numbers.append(&mut get_part_numbers(line_above.as_deref(), &line, None)?);

    println!("Part 1: {}", part_numbers.into_iter().sum::<u32>());

    Ok(())
}

fn get_part_numbers(line_above: Option<&str>, line: &str, line_below: Option<&str>) -> io::Result<Vec<u32>> {
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
    let parsed_items = parse_line(&line)?;

    for parsed_item in parsed_items {
        let mut has_adjacent_symbol = false;

        // check for adjacent symbol in line above
        if let Some(line_above) = line_above {
            let surrounding_range = 
                get_surrounding_range(&parsed_item.range, 0..line_above_len);
            
            if line_above.get(surrounding_range).unwrap().chars().any(|char| is_symbol(char)) {
                has_adjacent_symbol = true;
            }
        };

        // check for adjacent symbols in current line
        let surrounding_range = 
            get_surrounding_range(&parsed_item.range, 0..line_len);
        let chars_iterator = line.chars();
        let mut chars_iterator = chars_iterator.skip(surrounding_range.start);
        
        if parsed_item.range.start > 0 {
            if let Some(c) = chars_iterator.next() {
                if is_symbol(c) {
                    has_adjacent_symbol = true;
                }
            }
        }

        let mut chars_iterator = chars_iterator.skip(parsed_item.range.len());

        if let Some(c) = chars_iterator.next() {
            if is_symbol(c) {
                has_adjacent_symbol = true;
            }
        }

        // check for adjacent symbol in line below
        if let Some(line_below) = line_below {
            let surrounding_range = 
                get_surrounding_range(&parsed_item.range, 0..line_below_len);
            
                if line_below.get(surrounding_range).unwrap().chars().any(|char| is_symbol(char)) {
                    has_adjacent_symbol = true;
                }
        };

        if has_adjacent_symbol {
            if let AstItem::Number { value: parsed_number } = parsed_item.value {
                values.push(parsed_number);
            }
        }
    }

    Ok(values)
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

fn parse_line(line: &str) -> io::Result<Vec<ParsedItem>> {
    let mut items = vec![];

    let mut char_indices_iterator = line.char_indices().peekable();

    while let Some(mut index_and_character) = char_indices_iterator.next() {
        let mut range: Range<usize> = index_and_character.0..index_and_character.0 + 1;

        let parsed_item = 
            if is_symbol(index_and_character.1) {
                Ok(ParsedItem { 
                    value: AstItem::Symbol { value: index_and_character.1 }, 
                    range: range
                })
            } else if index_and_character.1 == '.' {
                Ok(ParsedItem {
                    value: AstItem::Dot,
                    range: range
                })
            } else if index_and_character.1.is_digit(10) {
                while index_and_character.1.is_digit(10) {
                    range.end = index_and_character.0 + 1;
        
                    match char_indices_iterator.peek() {
                        Some(c) => {
                            // prevent consuming the first non-digit character when trying to parse a number
                            if !c.1.is_digit(10) {
                                break;
                            }
                            
                            index_and_character = char_indices_iterator.next().unwrap();
                        }
                        None => { break }
                    }
                }
    
                match line[range.clone()].parse() {
                    Ok(value) => 
                        Ok(ParsedItem {
                            value: AstItem::Number { value: value },
                            range: range
                        }),
                    Err(err) => Err(io::Error::new(io::ErrorKind::Other, err))
                }
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "Unrecognized token at pos {index_and_character.0}"))
            };

        items.push(parsed_item?);
    }

    Ok(items)
}

#[cfg(test)]
mod test
{
    mod parse_line {
        use crate::{parse_line, AstItem, ParsedItem};

        #[test]
        fn it_parses_a_single_token() {
            assert_eq!(
                vec![ParsedItem { 
                        value: AstItem::Number { value: 42 }, 
                        range: 0..2}],

                parse_line("42").unwrap(), 
                "number"
            );

            assert_eq!(
                vec![ParsedItem { 
                        value: AstItem::Symbol { value: '*' }, 
                        range: 0..1}],

                parse_line("*").unwrap(), 
                "* symbol"
            );

            assert_eq!(
                vec![ParsedItem { 
                        value: AstItem::Dot, 
                        range: 0..1}],

                parse_line(".").unwrap(), 
                "dot"
            );
        }

        #[test]
        fn it_parses_multiple_tokens() {
            assert_eq!(
                vec![
                    ParsedItem { value: AstItem::Dot, range: 0..1},
                    ParsedItem { value: AstItem::Dot, range: 1..2},
                    ParsedItem { value: AstItem::Symbol { value: '$' }, range: 2..3 },
                    ParsedItem { 
                        value: AstItem::Number { value: 42 }, 
                        range: 3..5},
                    ParsedItem { value: AstItem::Symbol { value: '*' }, range: 5..6 },
                    ParsedItem { value: AstItem::Dot, range: 6..7},
                    ParsedItem { value: AstItem::Dot, range: 7..8}
                ],

                parse_line("..$42*..").unwrap(), 
                "single number enclosed with symbols or dots"
            );

            assert_eq!(
                vec![
                    ParsedItem { 
                        value: AstItem::Number { value: 1 }, 
                        range: 0..1},
                    ParsedItem { value: AstItem::Dot, range: 1..2},
                    ParsedItem { value: AstItem::Dot, range: 2..3},
                    ParsedItem { value: AstItem::Symbol { value: '$' }, range: 3..4 },
                    ParsedItem { 
                        value: AstItem::Number { value: 23 }, 
                        range: 4..6},
                    ParsedItem { value: AstItem::Symbol { value: '*' }, range: 6..7 },
                    ParsedItem { value: AstItem::Dot, range: 7..8},
                    ParsedItem { value: AstItem::Dot, range: 8..9},
                    ParsedItem { 
                        value: AstItem::Number { value: 42 }, 
                        range: 9..11}
                ],
                
                parse_line("1..$23*..42").unwrap(), 
                "multiple numbers mixed with symbols or dots"
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
                ).unwrap(),
                "single non-part number"
            );

            assert_eq!(
                vec![2],

                get_part_numbers(
                    None,
                    "*2",
                    None
                ).unwrap(),
                "single part number"
            );

            assert_eq!(
                vec![2, 3],

                get_part_numbers(
                    None,
                    "1.2*3.4",
                    None
                ).unwrap(),
                "single line with multiple numbers"
            );

            assert_eq!(
                vec![1, 3],

                get_part_numbers(
                    Some(   "......*"),
                            ".1.2.3.",
                    Some(   "$......")
                ).unwrap(),
                "multiple lines with some part numbers"
            );
        }
    }
}

