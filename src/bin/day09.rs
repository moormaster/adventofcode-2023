use std::io;

use adventofcode_2023::input_helper::read_lines;

fn main() -> io::Result<()> {
    let lines: Vec<String> = read_lines("input/day09")?.map(|e| e.unwrap()).collect();

    println!("Part 1: {}", lines.iter().map(|e| process(e)).sum::<isize>());
    println!("Part 2: {}", lines.iter().map(|e| process_part2(e)).sum::<isize>());

    Ok(())
}

fn process(line: &str) -> isize {
    let numbers = 
        line
            .split_whitespace()
            .map(|e| e.parse::<isize>().unwrap() )
            .collect();

    discrecte_extrapolation_behind_end(numbers)
}

fn process_part2(line: &str) -> isize {
    let numbers = 
        line
            .split_whitespace()
            .map(|e| e.parse::<isize>().unwrap() )
            .collect();

    discrecte_extrapolation_before_beginning(numbers)
}

fn discrecte_extrapolation_behind_end(numbers: Vec<isize>) -> isize {
    let derivatives = discrete_derivatives(&numbers);

    // extrapolate the next value behind the end of the n th derivative sequence
    // by adding the last value of the n+1 nth derivative sequence
    let next_value_of_first_derivative = 
        derivatives.iter().rev().fold(
            0isize, 
            |acc, derivative| derivative.last().unwrap() + acc 
        );

    // extrapolate next number
    numbers.last().unwrap() + next_value_of_first_derivative
}

fn discrecte_extrapolation_before_beginning(numbers: Vec<isize>) -> isize {
    let derivatives = discrete_derivatives(&numbers);

    // extrapolate the next value before the beginning of the n th derivative sequence
    // by substracting the first value of the n+1 nth derivative sequence
    let next_value_of_first_derivative = 
        derivatives.iter().rev().fold(
            0isize, 
            |acc, derivative| derivative.first().unwrap() - acc 
        );

    // extrapolate next number
    numbers.first().unwrap() - next_value_of_first_derivative
}

fn discrete_derivatives(numbers: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut last_derivative = numbers;

    let mut discrete_derivatives = Vec::new();
    while !last_derivative.iter().all(|e| *e == 0)
    {
        discrete_derivatives.push(discrete_derivative(last_derivative));

        last_derivative = discrete_derivatives.last().unwrap();
    }

    discrete_derivatives
}

fn discrete_derivative(numbers: &Vec<isize>) -> Vec<isize> {
    numbers.windows(2).map(|e| e[1] - e[0]).collect()
}

#[cfg(test)]
mod test {
    mod process {
        use crate::process;
        use crate::process_part2;

        #[test]
        fn it_should_extrapolate_the_next_number_in_each_sequence() {
            assert_eq!(18, process("0 3 6 9 12 15"));
            assert_eq!(28, process("1 3 6 10 15 21"));
            assert_eq!(68, process("10 13 16 21 30 45"));
        }

        #[test]
        fn it_should_extrapolate_the_number_before_the_beginning_in_each_sequence() {
            assert_eq!(-3, process_part2("0 3 6 9 12 15"));
            assert_eq!(0, process_part2("1 3 6 10 15 21"));
            assert_eq!(5, process_part2("10 13 16 21 30 45"));
        }
    }
}