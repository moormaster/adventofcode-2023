use std::{cmp, io, fmt::Display, str::FromStr};

use adventofcode_2023::input_helper::read_lines;

fn main() -> io::Result<()> {
    let lines: Vec<String> = 
        read_lines("input/day07")?
            .map( |line_result| line_result.unwrap() )
            .collect();

    println!(
        "Part 1: {}",
        process_lines(lines)
            .iter()
            .enumerate()
            .map( |(pos, e)| (pos+1) as u32 * e.bid )
            .sum::<u32>()
    );

    Ok(())
}

fn process_lines(lines: Vec<String>) -> Vec<HandAndBid> {
    let mut hands: Vec<HandAndBid> = lines
        .iter()
        .map( |line| line.parse::<HandAndBid>().expect(&format!("Failed to parse line: '{}'", line)) )
        .collect();

    hands.sort_by_key(|e| e.hand.clone());  // sort by first card, second card, ...
    hands.sort_by_key(                           // sort by win-type - keeping the order for hands with identical win-type intact
        |e| e.hand.get_win_type() 
    );

    hands
}


#[derive(Debug)]
#[derive(PartialEq)]
struct HandAndBid {
    hand: Hand,
    bid: u32
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, cmp::Eq, PartialOrd, cmp::Ord)]
struct Hand {
    cards: [Card; 5]
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, cmp::Eq, PartialOrd, cmp::Ord)]
enum Card {
    Ace=14,
    King=13,
    Queen=12,
    Jack=11,
    Ten=10,
    Nine=9,
    Eight=8,
    Seven=7,
    Six=6,
    Five=5,
    Four=4,
    Three=3,
    Two=2
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, cmp::Eq, PartialOrd, cmp::Ord)]
enum WinType {
    FiveOfAKind=7,
    FourOfAKind=6,
    FullHouse=5,
    ThreeOfAKind=4,
    TwoPair=3,
    OnePair=2,
    HighCard=1
}

impl FromStr for HandAndBid {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = 
            s.split_once(" ")
            .ok_or(io::Error::new(io::ErrorKind::Other, format!("Missing delimiter ' ': '{}'", s)) )?;

        let hand = hand.parse::<Hand>()?;
        let bid = 
            bid.parse::<u32>()
            .or( Err(io::Error::new(io::ErrorKind::Other, format!("Failed to parse bid: '{}'", bid))) )?;

        Ok(HandAndBid {
            hand,
            bid
        })
    }
}

impl Hand {
    fn new(cards: Vec<Card>) -> Hand {
        Hand {
            cards: cards.try_into().expect("A hand must consist of exactly 5 cards!")
        }
    }

    fn get_win_type(&self) -> WinType {
        // count of 2's, 3's, ..., A's
        let mut card_count = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ,0 ,0];

        // count of single, pairs, triples, ...
        let mut set_of_size_count = [0, 0, 0, 0, 0];
        
        self.cards.iter().cloned().for_each(
            |card| card_count[(card as usize)-2] += 1 );
        card_count.iter()
            .for_each( |card_count| if *card_count > 0 { set_of_size_count[*card_count-1] += 1 } );

        match set_of_size_count {
            [_, _, _, _, 1] => WinType::FiveOfAKind,
            [_,_,_,1,_] => WinType::FourOfAKind,
            [_,1,1,_,_] => WinType::FullHouse,
            [_,0,1,_,_] => WinType::ThreeOfAKind,
            [_,2,_,_,_] => WinType::TwoPair,
            [_,1,0,_,_] => WinType::OnePair,
            _ => WinType::HighCard
        }
    }
}

impl FromStr for Hand {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards =
            s.chars()
                .map( |char| { format!("{}", char).parse::<Card>().unwrap() } )
                .collect();

        Ok(Hand::new(cards))
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Ace => { f.write_str("A")?; }
            Card::King => { f.write_str("K")?; }
            Card::Queen => { f.write_str("Q")?; }
            Card::Jack => { f.write_str("J")?; }
            Card::Ten => { f.write_str("T")?; }
            Card::Nine => { f.write_str("9")?; }
            Card::Eight => { f.write_str("8")?; }
            Card::Seven => { f.write_str("7")?; }
            Card::Six => { f.write_str("6")?; }
            Card::Five => { f.write_str("5")?; }
            Card::Four => { f.write_str("4")?; }
            Card::Three => { f.write_str("3")?; }
            Card::Two => { f.write_str("2")?; }
        }

        Ok(())
    }
}

impl FromStr for Card {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err( io::Error::new(io::ErrorKind::Other, "Cannot parse Card from empty string!") );
        } else if s.len() > 1 {
            return Err( io::Error::new(io::ErrorKind::Other, format!("Card not represented by exactly one character: '{}'", s)) );
        }

        match s.chars().next().unwrap() {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            c => Err(io::Error::new(io::ErrorKind::Other, format!("Unknown card: '{}'", c)) )
        }
    }
}

#[cfg(test)]
mod test {
    mod process_lines {
        use crate::{Hand, process_lines};

        const SAMPLE_INPUT: &str = 
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        
        #[test]
        fn it_should_sort_hands_by_rank() {
            assert_eq!(
                vec![
                    "32T3K".parse::<Hand>().unwrap(),
                    "KTJJT".parse::<Hand>().unwrap(),
                    "KK677".parse::<Hand>().unwrap(),
                    "T55J5".parse::<Hand>().unwrap(),
                    "QQQJA".parse::<Hand>().unwrap()
                ],

                process_lines(
                    SAMPLE_INPUT.split("\n").map( &str::to_string ).collect()
                ).iter()
                    .map( |hand| hand.hand.clone() )
                    .collect::<Vec<Hand>>(),
                "Hands must be sorted by rank - lowest rank first"
            );
        }
    }

    mod hand_get_win_type {
        use crate::{WinType, Hand};

        #[test]
        fn it_should_determine_the_win_type_for_each_hand() {
            assert_eq!(
                WinType::OnePair,

                "32T3K"
                    .parse::<Hand>().unwrap()
                    .get_win_type(),
                "failed to recognize correct win type"
            );

            assert_eq!(
                WinType::TwoPair,

                "KK677"
                    .parse::<Hand>().unwrap()
                    .get_win_type(),
                "failed to recognize correct win type"
            );

            assert_eq!(
                WinType::TwoPair,

                "KTJJT"
                    .parse::<Hand>().unwrap()
                    .get_win_type(),
                "failed to recognize correct win type"
            );

            assert_eq!(
                WinType::ThreeOfAKind,

                "T55J5"
                    .parse::<Hand>().unwrap()
                    .get_win_type(),
                "failed to recognize correct win type"
            );

            assert_eq!(
                WinType::ThreeOfAKind,

                "QQQJA"
                    .parse::<Hand>().unwrap()
                    .get_win_type(),
                "failed to recognize correct win type"
            );

            assert_eq!(
                WinType::FullHouse,

                "QQQAA"
                    .parse::<Hand>().unwrap()
                    .get_win_type(),
                "failed to recognize correct win type"
            );

            assert_eq!(
                WinType::FourOfAKind,

                "QQQQ2"
                    .parse::<Hand>().unwrap()
                    .get_win_type(),
                "failed to recognize correct win type"
            );

            assert_eq!(
                WinType::FiveOfAKind,

                "22222"
                    .parse::<Hand>().unwrap()
                    .get_win_type(),
                "failed to recognize correct win type"
            );
        }
    }
}