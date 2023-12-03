use adventofcode_2023::input_helper;

fn to_calibration_value(line : &str) -> Result<u8, &str> {
    let mut first_digit: Option<u8> = None;
    let mut last_digit: Option<u8> = None;

    for index_char in line.char_indices() {
        let index = index_char.0;
        let char = index_char.1;

        let mut digit: Option<u8> = None;
        
        if char.is_digit(10) {
            digit = Some(char.to_digit(10).unwrap() as u8);
        }

        if line[index..].starts_with("one") {
            digit = Some(1);
        } else if line[index..].starts_with("two") {
            digit = Some(2);
        } else if line[index..].starts_with("three") {
            digit = Some(3);
        } else if line[index..].starts_with("four") {
            digit = Some(4);
        } else if line[index..].starts_with("five") {
            digit = Some(5);
        } else if line[index..].starts_with("six") {
            digit = Some(6);
        } else if line[index..].starts_with("seven") {
            digit = Some(7);
        } else if line[index..].starts_with("eight") {
            digit = Some(8);
        } else if line[index..].starts_with("nine") {
            digit = Some(9);
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

    let mut sum = 0u32;
    for line in lines {
        sum += to_calibration_value(&line?).unwrap() as u32;
    }

    println!("{sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    mod to_calibration_value {
        use crate::to_calibration_value;

        #[test]
        fn it_fails_without_any_digits() {
            assert_eq!(to_calibration_value("aWord"), Err("no digits"))
        }

        #[test]
        fn it_concatenates_the_only_digit_twice() {
            assert_eq!(to_calibration_value("word1word"), Ok(11))
        }

        #[test]
        fn it_concatenates_first_digit_with_last_digit() {
            assert_eq!(to_calibration_value("word1word2word3word"), Ok(13))
        }

        #[test]
        fn it_recognizes_words_as_digit() {
            assert_eq!(to_calibration_value("one2three"), Ok(13))
        }
    }
}