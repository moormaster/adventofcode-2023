use std::{io, ops};

use adventofcode_2023::input_helper::read_lines;

fn main() -> io::Result<()> {
    let lines: Vec<String> = read_lines("input/day06")
        .unwrap()
        .map( |line| line.unwrap() )
        .collect();

    let scoreboard = parse_times_and_records_part1(&lines[0], &lines[1]);

    println!(
        "Part 1: {}",
        scoreboard
            .iter()
            .map(
                |record|
                
                get_winning_acceleration_ms(record.time_ms, record.distance_record_mm)
                .and_then(
                    |range|

                    Some(range.end()-range.start()+1)
                )
                .unwrap_or_default()
             )
             .product::<u64>()
    );

    let scoreboard_record = parse_time_and_record_part2(&lines[0], &lines[1]);

    println!(
        "Part 1: {}",
        get_winning_acceleration_ms(scoreboard_record.time_ms, scoreboard_record.distance_record_mm)
            .and_then(
                |range| 
                
                Some(range.end()-range.start()+1) 
            )
            .unwrap_or_default()
    );

    Ok(())
}

#[derive(Debug)]
struct ScoreboardRecord {
    time_ms: u64,
    distance_record_mm: u64
}

fn parse_times_and_records_part1(line_time: &str, line_distance_record: &str) -> Vec<ScoreboardRecord> {
    let times_ms: Vec<u64> = line_time.split_whitespace()
        .skip(1)
        .map( |value| value.parse::<u64>().unwrap() )
        .collect();

    let distance_record_mm: Vec<u64> = line_distance_record.split_whitespace()
        .skip(1)
        .map( |value| value.parse::<u64>().unwrap() )
        .collect();

    times_ms.iter()
        .zip( distance_record_mm.iter() )
        .map( |pair| ScoreboardRecord { time_ms: *pair.0, distance_record_mm: *pair.1 } )
        .collect()
}

fn parse_time_and_record_part2(line_time: &str, line_distance_record: &str) -> ScoreboardRecord {
    let time_ms: u64 = line_time.split_whitespace()
        .skip(1)
        .map( |value| value.to_string() )
        .reduce( |acc, e| acc.to_string() + &e ).unwrap()
        .parse().unwrap();

    let distance_record_mm: u64 = line_distance_record.split_whitespace()
        .skip(1)
        .map( |value| value.to_string() )
        .reduce( |acc, e| acc.to_string() + &e ).unwrap()
        .parse().unwrap();

    ScoreboardRecord {
        time_ms,
        distance_record_mm
    }
}

fn get_winning_acceleration_ms(time_ms: u64, distance_record_mm: u64) -> Option<ops::RangeInclusive<u64>> {
    let mut acceleration_time = 0;

    while acceleration_time < time_ms/2 && acceleration_time*(time_ms - acceleration_time) <= distance_record_mm{
        acceleration_time += 1;
    }

    if acceleration_time*(time_ms - acceleration_time) > distance_record_mm {
        Some(acceleration_time..=time_ms-acceleration_time)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    mod get_winning_ms {
        use crate::get_winning_acceleration_ms;

        #[test]
        fn it_should_return_ranges_of_ms_that_win_the_race() {
            // boats initial speed is 0 mm / ms
            // each ms the button gets hold the speed increases by 1 mm / ms
            // after releasing the button the speed is kept

            // so holding the button for 3 ms increases the speed to 1 mm / ms, to 2 mm / ms, to 3 mm / ms
            // the boat does not travel the first 3 ms
            // in the remaining ms it will travel with 3 mm / ms until the given time is up

            assert_eq!(
                Some(2..=5),

                get_winning_acceleration_ms(7, 9),
                "travelling for 5 ms with 2 mm / ms will beat the distance record of 9 mm, 
                 also travelling for 2 ms with 5 mm / ms will beat the distance record"
            );

            assert_eq!(
                Some(4..=11),

                get_winning_acceleration_ms(15, 40),
                "travelling for 11 ms with 4 mm / ms will beat the distance record of 40 mm, 
                 also travelling for 4 ms with 11 mm / ms will beat the distance record"
            );

            assert_eq!(
                Some(11..=19),

                get_winning_acceleration_ms(30, 200),
                "travelling for 19 ms with 11 mm / ms will beat the distance record of 200 mm, 
                 also travelling for 11 ms with 19 mm / ms will beat the distance record"
            );
        }

        #[test]
        fn it_should_return_none_for_distance_record_that_is_impossible_to_beat() {
            assert_eq!(
                None,

                get_winning_acceleration_ms(7, 12),
                "travelling for 4 ms with 3 mm / ms will get at most 12 mm far"
            );
        }
    }
}