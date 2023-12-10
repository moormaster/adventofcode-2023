use std::collections::{BTreeSet, HashSet};
use std::io;


use adventofcode_2023::input_helper::read_lines;

struct ScratchCardGame {
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

    let mut cards = vec![];
    
    for line in lines {
        let game = parse_game(&line?)?;
        
        cards.push(game);
    }

    let cards_count_and_worth = calculate_number_of_cards(&cards);

    println!(
        "Part 1: {}",
        cards
            .iter()
            .map( |game| game.get_worth() )
            .sum::<usize>());

    println!(
      "Part 2: {}",
      cards_count_and_worth
        .iter()
        .map( |count_and_card| count_and_card.0 )
        .sum::<usize>());

    Ok(())
}

fn calculate_number_of_cards(cards: &Vec<ScratchCardGame>) -> Vec<(usize, &ScratchCardGame)> {
    let mut count_and_cards = vec![];

    for card in cards {
        count_and_cards.push((1, card));
    }

    for i in 0..cards.len() {
        let worth = cards[i].get_number_of_winning_bets();

        for j in i+1..i+1+worth {
            count_and_cards[j].0 += count_and_cards[i].0;
        }
    }

    count_and_cards
}

fn parse_game(line: &str) -> io::Result<ScratchCardGame> {
    let mut card_and_numbers = line.split(":");

    // skip card id
    card_and_numbers.next();

    let winning_numbers_and_numbers_on_card =
        card_and_numbers
            .next()
            .ok_or( io::Error::new(
                        io::ErrorKind::Other,
                        format!("missing separator ':': '{}'", line)
            ))?;

    let (winning_numbers, numbers_on_card) = parse_card_numbers(winning_numbers_and_numbers_on_card)?;

    let winning_numbers = HashSet::from_iter(winning_numbers);
    let numbers_on_card = BTreeSet::from_iter(numbers_on_card);

    Ok(
        ScratchCardGame { 
            winning_numbers,
            numbers_on_card
        }
    )
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