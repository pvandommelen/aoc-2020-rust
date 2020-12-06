use std::fs;

use aoc_2020_rust::util::bench;

fn get_bitset(bytes: &[u8]) -> u32 {
    bytes.iter().fold(0, |bitset, &c| {
        bitset | if c != b'\n' { 
            1 << (c - b'a')
        } else { 0 }
    })
}

fn count_unique_characters_in_group(group: &[u8]) -> usize {
    return get_bitset(group).count_ones() as usize;
}

fn count_shared_characters_in_group(group: &[u8]) -> usize {
    let set: u32 = group.split(|&c| c == b'\n')
        .map(get_bitset)
        .fold(0xffffffff, |total_bitset, bitset| {
            total_bitset & bitset
        });

    return set.count_ones() as usize;
}

fn iterate_and_sum(input: &str, group_calculator: fn(&[u8]) -> usize) -> usize {
    return input.split("\n\n")
        .map(|group| {
            return group_calculator(group.as_bytes());
        })
        .sum();
}

fn main() {
    let input = fs::read_to_string("./src/day06/input.txt").expect("Unable to read input file");

    let part1 = bench::time_repeat("part 1", || iterate_and_sum(input.trim(), count_unique_characters_in_group), 1000);
    let part2 = bench::time_repeat("part 2", || iterate_and_sum(input.trim(), count_shared_characters_in_group), 1000);
    
    let together = bench::time_repeat("together", || {
        input.split("\n\n")
            .map(|group| {
                group.as_bytes().split(|&c| c == b'\n')
                    .map(get_bitset)
                    .fold((0, std::u32::MAX), |(a, b), bitset| {
                        (a | bitset, b & bitset)
                    })
            })
            .fold((0, 0), |(part1, part2), (a, b)| {
                (part1 + a.count_ones(), part2 + b.count_ones())
            })
    }, 1000);

    let prepared_bitsets: Vec<Vec<u32>> = bench::time_repeat("prepare bitsets", || {
        return input.split("\n\n")
            .map(|group| {
                group.as_bytes()
                    .split(|&c| c == b'\n')
                    .map(get_bitset)
                    .collect()
            })
            .collect();
    }, 1000);

    let part1_prepared_bitsets: usize = bench::time_repeat("part 1 - prepared counts", || prepared_bitsets.iter().map(|lines| {
        lines.iter().fold(0, |acc, bitset| acc | bitset).count_ones() as usize
    }).sum(), 1000);
    let part2_prepared_bitsets: usize = bench::time_repeat("part 2 - prepared counts", || prepared_bitsets.iter().map(|lines| {
        lines.iter().skip(1).fold(lines[0], |acc, bitset| acc & bitset).count_ones() as usize
    }).sum(), 1000);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Together: {} {}", together.0, together.1);
    println!("Part 1 - prepared bitsets: {}", part1_prepared_bitsets);
    println!("Part 2 - prepared bitsets: {}", part2_prepared_bitsets);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = 
"abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn will_count_unique_characters_in_group() {
        assert_eq!(count_unique_characters_in_group("abc".as_bytes()), 3);
        assert_eq!(count_unique_characters_in_group("a\nb\nc".as_bytes()), 3);
        assert_eq!(count_unique_characters_in_group("ab\nac".as_bytes()), 3);
        assert_eq!(count_unique_characters_in_group("a\na\na\na".as_bytes()), 1);
        assert_eq!(count_unique_characters_in_group("b".as_bytes()), 1);
    }
    
    #[test]
    fn will_count_part1_example() {
        assert_eq!(iterate_and_sum(EXAMPLE_INPUT, count_unique_characters_in_group), 11);
    }
    #[test]
    fn will_count_shared_characters_in_group() {
        assert_eq!(count_shared_characters_in_group("abc".as_bytes()), 3);
        assert_eq!(count_shared_characters_in_group("a\nb\nc".as_bytes()), 0);
        assert_eq!(count_shared_characters_in_group("ab\nac".as_bytes()), 1);
        assert_eq!(count_shared_characters_in_group("a\na\na\na".as_bytes()), 1);
        assert_eq!(count_shared_characters_in_group("b".as_bytes()), 1);
    }
    
    #[test]
    fn will_count_part2_example() {
        assert_eq!(iterate_and_sum(EXAMPLE_INPUT, count_shared_characters_in_group), 6);
    }
}