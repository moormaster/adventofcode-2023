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

    let items_from_first_line = parse_line(&lines.next().unwrap()?)?;
    let items_from_second_line = parse_line(&lines.next().unwrap()?)?;

    let mut part_numbers = vec![];
    let mut wheels = vec![];

    let mut items_above: Option<Vec<ParsedItem>> = None;
    let mut items = Some(items_from_first_line);
    let mut items_below = Some(items_from_second_line);

    while items.is_some() {
        part_numbers.append(&mut get_part_numbers(items_above.as_ref(), items.as_ref().unwrap(), items_below.as_ref())?);
        wheels.append(&mut get_wheels(items_above.as_ref(), items.as_ref().unwrap(), items_below.as_ref())?);

        items_above = items;
        items = items_below;
        items_below = lines.next()
                        .and_then(|line_read| {line_read.ok()})
                        .and_then(|line_read| {parse_line(&line_read).ok()});
    }

    println!("Part 1: {}", part_numbers.into_iter().sum::<u32>());
    println!(
        "Part 2: {}", 
        wheels.into_iter()
            .map(|wheel| {wheel.0 * wheel.1})
            .sum::<u32>()
    );

    Ok(())
}

fn get_part_numbers(items_above: Option<&Vec<ParsedItem>>, items: &Vec<ParsedItem>, items_below: Option<&Vec<ParsedItem>>) -> io::Result<Vec<u32>> {
    let mut values = vec![];
    let mut items = items.iter();

    let mut item_before: Option<&ParsedItem> = None;
    let mut item = items.next();
    let mut item_behind = items.next();

    while item.is_some() {
        let mut has_adjacent_symbol = false;
        let surrounding_range = get_surrounding_range(&item.unwrap().range);

        // check for adjacent symbol in line above
        if let Some(items_above) = items_above {
            for item_above in items_above {
                match item_above {
                    ParsedItem{ value: AstItem::Symbol { value: _ }, range } => {
                        if surrounding_range.contains(&range.start) || surrounding_range.contains(&(range.end-1)) {
                            has_adjacent_symbol = true;
                        }
                    },
                    _ => {}
                }
            }
        };

        // check for adjacent symbols in current line
        if let Some(item_before) = item_before {
            if let ParsedItem { range: _, value: AstItem::Symbol { value: _ } } = item_before {
                has_adjacent_symbol = true;
            }
        }

        if let Some(item_behind) = item_behind {
            if let ParsedItem { range: _, value: AstItem::Symbol { value: _ } } = item_behind {
                has_adjacent_symbol = true;
            }
        }

        // check for adjacent symbol in line below
        if let Some(items_below) = items_below {
            for item_below in items_below {
                match item_below {
                    ParsedItem{ value: AstItem::Symbol { value: _ }, range } => {
                        if surrounding_range.contains(&range.start) || surrounding_range.contains(&(range.end-1)) {
                            has_adjacent_symbol = true;
                        }
                    },
                    _ => {}
                }
            }
        };

        if has_adjacent_symbol {
            if let AstItem::Number { value: parsed_number } = item.unwrap().value {
                values.push(parsed_number);
            }
        }

        // advance variables for item before, current item, item behind
        item_before = item;
        item = item_behind;
        // ... and read next item in line
        item_behind = items.next();
    }

    Ok(values)
}

fn get_wheels(items_above: Option<&Vec<ParsedItem>>, items: &Vec<ParsedItem>, items_below: Option<&Vec<ParsedItem>>) -> io::Result<Vec<(u32, u32)>> {
    let mut wheels: Vec<(u32, u32)> = vec![];
    let mut items = items.iter();

    let mut item_before: Option<&ParsedItem> = None;
    let mut item = items.next();
    let mut item_behind = items.next();

    while item.is_some() {
        let mut adjacent_numbers = vec![0u32; 0];

        if let Some(item) = item {
            if let AstItem::Symbol { value: _ } = item.value {
                let surrounding_range = get_surrounding_range(&item.range);

                // find adjacent numbers in line above
                if let Some(items_above) = items_above {
                    for item_above in items_above {
                        if let AstItem::Number { value } = item_above.value {
                            if surrounding_range.contains(&item_above.range.start) || surrounding_range.contains(&(item_above.range.end-1)) {
                                adjacent_numbers.push(value);
                            }
                        }
                    }
                }

                // check for adjacent numbers in current line
                if let Some(item_before) = item_before {
                    if let AstItem::Number { value } = item_before.value {
                        adjacent_numbers.push(value);
                    }
                }

                if let Some(item_behind) = item_behind {
                    if let AstItem::Number { value } = item_behind.value {
                        adjacent_numbers.push(value);
                    }
                }

                // find adjacent numbers in line below
                if let Some(items_below) = items_below {
                    for item_below in items_below {
                        if let AstItem::Number { value } = item_below.value {
                            if surrounding_range.contains(&item_below.range.start) || surrounding_range.contains(&(item_below.range.end-1)) {
                                adjacent_numbers.push(value);
                            }
                        }
                    }
                }

                if adjacent_numbers.len() == 2 {
                    wheels.push((adjacent_numbers[0], adjacent_numbers[1]));
                }
            }
        }

        // advance variables for item before, current item, item behind
        item_before = item;
        item = item_behind;
        // ... and read next item in line
        item_behind = items.next();
    }

    Ok(wheels)
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn get_surrounding_range(range: &Range<usize>) -> Range<usize> {
    Range { 
        start: if range.start > 0 { range.start-1 } else { 0 },
        end: range.end+1
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
        use crate::{get_part_numbers, parse_line};

        #[test]
        fn it_returns_numbers_adjacent_to_a_symbol() {
            assert_eq!(
                vec![0u32; 0],

                get_part_numbers(
                    None,
                    &parse_line("2").unwrap(),
                    None
                ).unwrap(),
                "single non-part number"
            );

            assert_eq!(
                vec![2],

                get_part_numbers(
                    None,
                    &parse_line("*2").unwrap(),
                    None
                ).unwrap(),
                "single part number"
            );

            assert_eq!(
                vec![2, 3],

                get_part_numbers(
                    None,
                    &parse_line("1.2*3.4").unwrap(),
                    None
                ).unwrap(),
                "single line with multiple numbers"
            );

            assert_eq!(
                vec![1, 3],

                get_part_numbers(
                    Some(   &parse_line("......*").unwrap()),
                            &parse_line(".1.2.3.").unwrap(),
                    Some(   &parse_line("$......").unwrap())
                ).unwrap(),
                "multiple lines with some part numbers"
            );
        }

        mod get_wheels {
            use crate::{get_wheels, parse_line};

            #[test]
            fn it_returns_a_single_wheel() {
                assert_eq!(
                    vec![
                        (23, 42)
                    ],

                    get_wheels(
                        None,
                        &parse_line("23*42").unwrap(),
                        None
                    ).unwrap(),
                    "single wheel within the same line"
                );

                assert_eq!(
                    vec![
                        (42, 123)
                    ],

                    get_wheels(
                        Some(   &parse_line("42....").unwrap()),
                                &parse_line("..*...").unwrap(),
                        Some(   &parse_line("...123").unwrap())
                    ).unwrap(),
                    "single wheel with upper left and lower right number"
                );

                assert_eq!(
                    vec![
                        (123, 42)
                    ],

                    get_wheels(
                        Some(   &parse_line("...123").unwrap()),
                                &parse_line("..*...").unwrap(),
                        Some(   &parse_line("42....").unwrap())
                    ).unwrap(),
                    "single wheel with upper right and lower left number"
                );
            }

            #[test]
            fn it_returns_multiple_wheels() {
                assert_eq!(
                    vec![
                        (42, 123)
                    ],

                    get_wheels(
                        Some(   &parse_line("1..42.....6").unwrap()),
                                &parse_line(".*...*....*").unwrap(),
                        Some(   &parse_line("2.3...123..").unwrap())
                    ).unwrap(),
                    "wheel enclosed in non-wheels"
                );
            }
        }
    }
}

