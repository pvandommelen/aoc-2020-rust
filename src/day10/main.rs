use std::fs;
use aoc_2020_rust::util::bench;
use aoc_2020_rust::util::parser;

use nom::{character::complete::newline, combinator::{all_consuming, opt}, multi::many1, sequence::terminated};

type PreparedInput = Vec<u32>;

fn prepare_input(input: &str) -> PreparedInput {
    let mut numbers = all_consuming(many1(
        terminated(
            parser::parse_decimal_u32,
            opt(newline)
        )
    ))(input.as_bytes()).expect("").1;

    numbers.sort();
    numbers
}

fn part1(input: &PreparedInput) -> u32 {
    assert!(input[0] <= 3);

    let mut count_1 = input[0];
    let mut count_3 = 1;

    for i in 0..input.len()-1 {
        let diff = input[i + 1] - input[i];
        match diff {
            1 => { count_1 += 1 }
            3 => { count_3 += 1 }
            _ => {}
        };
    }

    count_1 * count_3
}
fn part2(input: &PreparedInput) -> u64 {
    let largest = input[input.len() - 1];
    let mut combinations: Vec<u64> = vec![0; largest as usize + 1];
    combinations[0] = 1;

    for &jolt in input.iter() {
        let mut n: u64 = 0;
        n += if jolt < 1 { 0 } else {combinations[jolt as usize - 1]};
        n += if jolt < 2 { 0 } else {combinations[jolt as usize - 2]};
        n += if jolt < 3 { 0 } else {combinations[jolt as usize - 3]};
        combinations[jolt as usize] = n;
    }

    combinations[combinations.len() - 1]
}

fn main() {
    let input = fs::read_to_string("./src/day10/input.txt").expect("Unable to read input file");

    let prepared_input = bench::time("input preparation", || prepare_input(&input));
    let part1 = bench::time("part 1", || part1(&prepared_input));
    let part2 = bench::time("part 2", || part2(&prepared_input));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: &str = 
"16
10
15
5
1
11
7
19
6
12
4";

    const EXAMPLE_INPUT2: &str = 
"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    
    #[test]
    fn part1_example1() {
        assert_eq!(part1(&prepare_input(EXAMPLE_INPUT1)), 7 * 5);
    }
    
    #[test]
    fn part1_example2() {
        assert_eq!(part1(&prepare_input(EXAMPLE_INPUT2)), 22 * 10);
    }
    
    #[test]
    fn part2_example1() {
        assert_eq!(part2(&prepare_input(EXAMPLE_INPUT1)), 8);
    }
    
    #[test]
    fn part2_example2() {
        assert_eq!(part2(&prepare_input(EXAMPLE_INPUT2)), 19208);
    }
}