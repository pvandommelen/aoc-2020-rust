use std::{collections::HashSet, fs};
use aoc_2020_rust::util::bench;
use aoc_2020_rust::util::parser;

use nom::{character::complete::newline, combinator::{all_consuming, opt}, multi::many1, sequence::terminated};

fn prepare_input(input: &str) -> Vec<u64> {
    all_consuming(many1(
        terminated(
            parser::parse_decimal_u64,
            opt(newline)
        )
    ))(input.as_bytes()).expect("").1
}

fn part1(input: &Vec<u64>, preamble_len: usize) -> u64 {
    let mut working_set: HashSet<u64> = HashSet::new();
    let mut working_vec: Vec<u64> = Vec::new();
    let mut vec_position: usize = 0;

    let found_nonexistent_sum = input.iter().find(|&&number| {
        assert!(!working_set.contains(&number));

        if working_vec.len() < preamble_len {
            working_vec.insert(working_vec.len(), number);
            working_set.insert(number);
            false
        } else {
            let sum_exists = working_vec.iter().find(|&&existing_number| {
                number > existing_number && working_set.contains(&(number - existing_number))
            }).is_some();

            working_set.remove(&working_vec[vec_position]);
            working_set.insert(number);
            working_vec[vec_position] = number;
            vec_position = (vec_position + 1) % preamble_len;
            
            !sum_exists
        }
    });
    *found_nonexistent_sum.unwrap()
}
fn part2(input: &Vec<u64>, preamble_len: usize) -> u64 {
    let expected_sum = part1(input, preamble_len);

    let mut working_vec: Vec<u64> = Vec::new();

    input.iter().find(|&&num| {
        working_vec.insert(working_vec.len(), num);
        let mut sum: u64;
        loop {
            sum = working_vec.iter().sum();
            if sum <= expected_sum {
                break;
            }
            working_vec.remove(0);
        }
        sum == expected_sum
    });

    assert!(working_vec.len() > 1);

    let min = working_vec.iter().min().unwrap();
    let max = working_vec.iter().max().unwrap();
    min + max
}

fn main() {
    let input = fs::read_to_string("./src/day09/input.txt").expect("Unable to read input file");

    let prepared_input = bench::time("input preparation", || prepare_input(&input));
    let part1 = bench::time("part 1", || part1(&prepared_input, 25));
    let part2 = bench::time("part 2", || part2(&prepared_input, 25));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = 
"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    
    #[test]
    fn part1_example() {
        assert_eq!(part1(&prepare_input(EXAMPLE_INPUT), 5), 127);
    }
    
    #[test]
    fn part2_example() {
        assert_eq!(part2(&prepare_input(EXAMPLE_INPUT), 5), 62);
    }
}