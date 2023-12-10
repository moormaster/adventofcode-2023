use std::collections::{BTreeSet, HashSet};
use std::io;


use adventofcode_2023::input_helper::read_lines;

struct ScratchCardGame {
    id: u32,
    winning_numbers: HashSet<u32>,

    // we want to iterate over numbers on card
    // this is cheaper on a BTreeSet than on a sparsely filled HashSet
    numbers_on_card: BTreeSet<u32>
}

impl ScratchCardGame{
    pub fn get_number_of_winning_bets(&self) -> usize {
        self.numbers_on_card.iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }

    pub fn get_worth(&self) -> usize {
        let number_of_winning_bets = self.get_number_of_winning_bets();

        if number_of_winning_bets > 0 {
            1<<(number_of_winning_bets-1)
        } else {
            0
        }
    }
}

fn main() -> io::Result<()> {
    let lines = read_lines("input/day04").unwrap();

    let mut card_worths = vec![0usize; 0];
    for line in lines {
        let game = parse_game(&line?)?;
        
        card_worths.push(game.get_worth())
    }

    println!(
        "Part 1: {}",
        card_worths
            .iter()
            .sum::<usize>());

    Ok(())
}

fn parse_game(line: &str) -> io::Result<ScratchCardGame> {
    let mut card_and_numbers = line.split(":");

    let card_id = parse_card_id(card_and_numbers.next().unwrap())?;

    let winning_numbers_and_numbers_on_card =
        card_and_numbers
            .next()
            .ok_or( io::Error::new(
                        io::ErrorKind::Other,
                        format!("missing separator ':': '{}'", line)
            ))?;

    let (winning_numbers, numbers_on_card) = parse_card_numbers(winning_numbers_and_numbers_on_card)?;

    // TODO: is there a shorter way to construct a HashSet from a Vec?
    let winning_numbers = 
        {
            let mut hash_set = HashSet::new();
            hash_set.extend(winning_numbers);
            hash_set
        };

    // TODO: is there a shorter way to construct a BTreeSet from a Vec?
    let numbers_on_card = 
        {
            let mut hash_set = BTreeSet::new();
            hash_set.extend(numbers_on_card);
            hash_set
        };

    Ok(
        ScratchCardGame { 
            id: card_id,
            winning_numbers,
            numbers_on_card
        }
    )
}

fn parse_card_id(line_part: &str) -> io::Result<u32> {
    let mut card_and_id = 
        line_part
            .split(" ")
            .filter(|token|
                token
                    .chars()
                    .count() > 0
            );
    if card_and_id.next().unwrap() != "Card" {
        return Err(io::Error::new(io::ErrorKind::Other, "Line is expected to start with 'Card'"));
    }

    let card_id_str = 
        card_and_id
            .next()
            .ok_or( io::Error::new(
                            io::ErrorKind::Other, 
                            format!("Missing separator ' ' between 'Card' and id: '{}'", line_part)
                    ) )?;

    let card_id = 
        card_id_str
            .parse::<u32>()
            .map_err( |_| 
                        io::Error::new(
                            io::ErrorKind::Other,
                            format!("Failed to parse id: '{card_id_str}'")
                        )
                    )?;

    Ok(card_id)
}

fn parse_card_numbers(line_part: &str) -> io::Result<(Vec<u32>, Vec<u32>)> {
    let mut winning_numbers_and_numbers_on_card = line_part.split("|");

    let winning_numbers = winning_numbers_and_numbers_on_card.next().unwrap();
    let winning_numbers = parse_numbers(winning_numbers)?;

    let numbers_on_card =
        winning_numbers_and_numbers_on_card
            .next()
            .ok_or( io::Error::new(io::ErrorKind::Other, format!("missing separator '|' in number information: '{}'", line_part)) )?;
    let numbers_on_card = parse_numbers(numbers_on_card)?;

    Ok((winning_numbers, numbers_on_card))
}

fn parse_numbers(line_part: &str) -> io::Result<Vec<u32>> {
    line_part
        .split(" ")
        // filter out "empty" parts caused by indentation of values
        .filter( |str|
            str
                .chars()
                .count() > 0
        )
        .map( |token|
            token
                .parse::<u32>()
                .map_err(|_|
                    io::Error::new(io::ErrorKind::Other, format!("failed to parse number: '{}'", token))
                )
        )
        .collect()
}

#[cfg(test)]
mod test {
    mod parse_game {
        use std::collections::{HashSet, BTreeSet};

        use crate::parse_game;

        #[test]
        fn it_should_parse_card_id() {
            assert_eq!(
                100,

                parse_game("Card 100: |").unwrap().id,
                "card id"
            );

            assert_eq!(
                1,

                parse_game("Card   1: |").unwrap().id,
                "indented card id"
            );
        }

        #[test]
        fn it_should_parse_winning_numbers() {
            assert_eq!(
                HashSet::new(),

                parse_game("Card 1: |").unwrap().winning_numbers,
                "no winning numbers and no numbers on cards"
            );

            assert_eq!(
                HashSet::from([42]),

                parse_game("Card 1: 42 |").unwrap().winning_numbers,
                "single winning number"
            );

            assert_eq!(
                HashSet::from([1]),

                parse_game("Card 1:  1 |").unwrap().winning_numbers,
                "single winning number indented"
            );

            assert_eq!(
                HashSet::from([1, 2, 10]),

                parse_game("Card 1:  1  2  10 |").unwrap().winning_numbers,
                "multiple winning numbers"
            );
        }

        #[test]
        fn it_should_parse_numbers_on_cards() {
            assert_eq!(
                BTreeSet::from([42]),

                parse_game("Card 1: | 42").unwrap().numbers_on_card,
                "single number on card"
            );

            assert_eq!(
                BTreeSet::from([1]),

                parse_game("Card 1: |  1").unwrap().numbers_on_card,
                "single number on card indented"
            );

            assert_eq!(
                BTreeSet::from([1, 2, 10]),

                parse_game("Card 1: |  1  2  10").unwrap().numbers_on_card,
                "multiple numbers on card"
            );
        }
    }
}