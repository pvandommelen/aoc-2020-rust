use std::fs;
use aoc_2020_rust::util::bench;

type PreparedInput = str;

fn prepare_input(input: &str) -> &PreparedInput {
    input
}

fn part1(input: &PreparedInput) -> u32 {
    0
}
fn part2(input: &PreparedInput) -> u32 {
    0
}

fn main() {
    let input = fs::read_to_string("./src/dayXX/input.txt").expect("Unable to read input file");

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
"";
    
    #[test]
    fn will_count_part1_example() {
        assert_eq!(part1(prepare_input(EXAMPLE_INPUT)), 0);
    }
    
    #[test]
    fn will_count_part2_example() {
        assert_eq!(part2(prepare_input(EXAMPLE_INPUT)), 0);
    }
}