use std::io;

use adventofcode_2023::input_helper::{self, read_lines};

fn main() -> io::Result<()> {
    let lines: Vec<String> = read_lines("input/day09")?.map(|e| e.unwrap()).collect();

    println!("Part 1: {}", lines.iter().map(|e| process(e)).sum::<isize>());

    Ok(())
}

fn process(line: &str) -> isize {
    let numbers = 
        line
            .split_whitespace()
            .map(|e| e.parse::<isize>().unwrap() )
            .collect();

    discrecte_extrapolation(numbers)
}

fn discrecte_extrapolation(numbers: Vec<isize>) -> isize {
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
    let mut last_entry = numbers[0];
    numbers.iter().skip(1).map(
        |e| { 
            let diff = *e - last_entry; 
            last_entry = *e; 
            diff 
        })
        .collect()
}

#[cfg(test)]
mod test {
    mod process {
        use crate::process;

        #[test]
        fn it_should_extrapolate_the_next_number_in_each_sequence() {
            assert_eq!(18, process("0 3 6 9 12 15"));
            assert_eq!(28, process("1 3 6 10 15 21"));
            assert_eq!(68, process("10 13 16 21 30 45"));
        }
    }
}