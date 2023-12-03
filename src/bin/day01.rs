use adventofcode_2023::input_helper;

fn process_line_part1(line : &str) -> Result<u8, &str> {
    let mut first_digit: Option<u8> = None;
    let mut last_digit: Option<u8> = None;

    for char in line.chars() {
        let mut digit: Option<u8> = None;
        
        if char.is_digit(10) {
            digit = Some(char.to_digit(10).unwrap() as u8);
        }

        if let Some(digit) = digit {
            if first_digit == None {
                first_digit = Some(digit);
            }
            
            last_digit = Some(digit);
        }
    }

    if let Some(first_digit) = first_digit {
        Ok(first_digit*10 + last_digit.unwrap())
    } else {
        Err("no digits")
    }
}

fn process_line_part2(line : &str) -> Result<u8, &str> {
    let mut first_digit: Option<u8> = None;
    let mut last_digit: Option<u8> = None;

    let digit_words: [(u8, &str); 9] =
        [   (1, "one"),
            (2, "two"),
            (3, "three"),
            (4, "four"),
            (5, "five"),
            (6, "six"),
            (7, "seven"),
            (8, "eight"),
            (9, "nine")];

    for index_char in line.char_indices() {
        let index = index_char.0;
        let char = index_char.1;

        let mut digit: Option<u8> = None;
        
        if char.is_digit(10) {
            digit = Some(char.to_digit(10).unwrap() as u8);
        } else {
            for digit_word in digit_words {
                // for word (i.e. "one") ...
                if line[index..].starts_with(digit_word.1) {
                    // set digit to the corresponding digit value (i.e. 1)
                    digit = Some(digit_word.0);
                }
            }
        }

        if let Some(digit) = digit {
            if first_digit == None {
                first_digit = Some(digit);
            }
            
            last_digit = Some(digit);
        }
    }

    if let Some(first_digit) = first_digit {
        Ok(first_digit*10 + last_digit.unwrap())
    } else {
        Err("no digits")
    }
}

pub fn main() -> std::io::Result<()> {
    let lines = input_helper::read_lines("input/day01")?;
    let sum_part1: u32 = 
    lines.into_iter()
        .map(|line| { process_line_part1(&line.unwrap()).unwrap() as u32 } )
        .sum();

    println!("part1: {sum_part1}");

    let lines = input_helper::read_lines("input/day01")?;
    let sum_part2: u32 = 
        lines.into_iter()
            .map(|line| { process_line_part2(&line.unwrap()).unwrap() as u32 } )
            .sum();

    println!("part2: {sum_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    mod process_line_part1 {
        use crate::process_line_part1;

        #[test]
        fn it_fails_without_any_digits() {
            assert_eq!(process_line_part1("aWord"), Err("no digits"))
        }

        #[test]
        fn it_concatenates_the_only_digit_twice() {
            assert_eq!(process_line_part1("word1word"), Ok(11))
        }

        #[test]
        fn it_concatenates_first_digit_with_last_digit() {
            assert_eq!(process_line_part1("word1word2word3word"), Ok(13))
        }
    }

    mod process_line_part2 {
        use crate::process_line_part2;

        #[test]
        fn it_fails_without_any_digits() {
            assert_eq!(process_line_part2("aWord"), Err("no digits"))
        }

        #[test]
        fn it_concatenates_the_only_digit_twice() {
            assert_eq!(process_line_part2("word1word"), Ok(11))
        }

        #[test]
        fn it_concatenates_first_digit_with_last_digit() {
            assert_eq!(process_line_part2("word1word2word3word"), Ok(13))
        }

        #[test]
        fn it_recognizes_words_as_digit() {
            assert_eq!(process_line_part2("one2three"), Ok(13))
        }
    }
}