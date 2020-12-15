use std::collections::HashMap;
use aoc_2020_rust::util::bench;

fn run(input: &Vec<u32>, number: usize) -> u32 {
    const LIMIT: u32 = 1 << 22;

    let mut spoken_map = HashMap::new();
    let mut spoken_vec: Vec<u32> = vec![std::u32::MAX; LIMIT as usize]; //u32::MAX represents not filled
    let mut last_value = input[0];

    (1..input.len()).for_each(|i| {
        let value = input[i];
        spoken_vec[last_value as usize] = i as u32;
        last_value = value;
    });

    (input.len()..number).fold(last_value, |last_value, i| {
        if last_value < LIMIT {
            let last_spoken = std::mem::replace(&mut spoken_vec[last_value as usize], i as u32);
            if last_spoken == std::u32::MAX { 0 } else { i as u32 - last_spoken }
        } else {
            let last_spoken = spoken_map.insert(last_value, i);
            match last_spoken {
                None => 0,
                Some(position) => {
                    (i - position) as u32
                },
            }
        }
    })
}

fn main() {
    let input = vec![1, 0, 15, 2, 10, 13];
    let part1 = bench::time("part 1", || run(&input, 2020));
    let part2 = bench::time("part 2", || run(&input, 30000000));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1_example() {
        assert_eq!(run(&vec![0,3,6], 2020), 436);
        assert_eq!(run(&vec![1,3,2], 2020), 1);
        assert_eq!(run(&vec![2,1,3], 2020), 10);
        assert_eq!(run(&vec![1,2,3], 2020), 27);
        assert_eq!(run(&vec![2,3,1], 2020), 78);
        assert_eq!(run(&vec![3,2,1], 2020), 438);
        assert_eq!(run(&vec![3,1,2], 2020), 1836);
    }
}