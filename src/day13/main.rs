use std::fs;
use aoc_2020_rust::util::{bench, parser::parse_decimal_u64};
use nom::{branch::alt, bytes::complete::tag, character::complete::newline, combinator::{all_consuming, map, opt}, multi::many1, sequence::{separated_pair, terminated}};

type PreparedInput = (
    u64,
    Vec<Option<u64>>,
);

fn prepare_input(input: &str) -> PreparedInput {
    all_consuming(separated_pair(
        parse_decimal_u64,
        newline,
        many1(terminated(
            alt((
                map(tag("x"), |_| None),
                map(parse_decimal_u64, |number| Some(number)),
            )),
            opt(tag(","))
        )),
    ))(input.as_bytes()).expect("").1
}

fn part1(input: &PreparedInput) -> u64 {
    let timestamp = input.0;
    let best = input.1.iter()
        .filter_map(|&option| option)
        .map(|bus_id| {
            let bus_waiting_time = bus_id - (timestamp + bus_id) % bus_id;
            (bus_id, bus_waiting_time)
        })
        .min_by_key(|(_, a)| {
            *a
        })
        .unwrap();
    
    best.0 * best.1
}

fn run2(ids: Vec<u64>) -> u64 {
    let mut current: u64 = 0;
    let mut ids_multiplied: u64 = 1;

    ids.iter().enumerate().for_each(|(i, &bus_id)| {
        if bus_id == 1 {
            return;
        }
        loop {
            if (current + i as u64) % bus_id == 0 {
                ids_multiplied *= bus_id;
                break;
            }
            current += ids_multiplied;
        }
    });

    current
}

fn part2(input: &PreparedInput) -> u64 {
    run2(input.1.iter().map(|option| {
        match option {
            None => 1,
            Some(number) => *number,
        }
    }).collect())
}

fn main() {
    let input = fs::read_to_string("./src/day13/input.txt").expect("Unable to read input file");

    let prepared_input = bench::time("input preparation", || prepare_input(&input));
    let part1 = bench::time("part 1", || part1(&prepared_input));
    let part2 = bench::time("part 2", || part2(&prepared_input));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = 
"939
7,13,x,x,59,x,31,19";
    
#[test]
    fn parse_example() {
        assert_eq!(prepare_input(EXAMPLE_INPUT), (
            939,
            vec![
                Some(7),
                Some(13),
                None,
                None,
                Some(59),
                None,
                Some(31),
                Some(19)
            ],
        ));
    }
    
    #[test]
    fn part1_example() {
        assert_eq!(part1(&prepare_input(EXAMPLE_INPUT)), 295);
    }
    
    #[test]
    fn part2_example() {
        assert_eq!(part2(&prepare_input(EXAMPLE_INPUT)), 1068781);
    }

    #[test]
    fn run2_examples() {
        assert_eq!(run2(vec![17, 1]), 0);
        assert_eq!(run2(vec![7, 13]), 77);
        assert_eq!(run2(vec![7, 1, 13]), 63);

        assert_eq!(run2(vec![17, 1, 13, 19]), 3417);
        assert_eq!(run2(vec![67, 7, 59, 61]), 754018);
        assert_eq!(run2(vec![67, 1, 7, 59, 61]), 779210);
        assert_eq!(run2(vec![67, 7, 1, 59, 61]), 1261476);
        assert_eq!(run2(vec![1789, 37, 47, 1889]), 1202161486);
        
        assert_eq!(run2(vec![7, 13, 1, 1, 59, 1, 31, 19]), 1068781);
    }
}