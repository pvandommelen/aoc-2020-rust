use std::{collections::HashMap, fs};
use aoc_2020_rust::util::{bench, parser::parse_decimal_u64};
use nom::{IResult, bytes::complete::tag, character::complete::{newline, one_of}, combinator::{all_consuming, map, opt}, multi::many1, sequence::{preceded, separated_pair, terminated, tuple}};

type Mask = (
    u64,
    u64,
);
type Instruction = (
    u64,
    u64,
);

type PreparedInput = Vec<(
    Mask,
    Vec<Instruction>,
)>;

fn parse_mask(i: &[u8]) -> IResult<&[u8], Mask> {
    preceded(
        tag("mask = "), 
        map(
            many1(one_of("01X")),
            |v| {
                (
                    v.iter().fold(0, |acc, &char| {
                        (acc << 1) + if char == '0' { 1 } else { 0 }
                    }),
                    v.iter().fold(0, |acc, &char| {
                        (acc << 1) + if char == '1' { 1 } else { 0 }
                    }),
                )
            }
        )
    )(i)
}

fn parse_instruction(i: &[u8]) -> IResult<&[u8], Instruction> {
    separated_pair(
        map(tuple((
            tag("mem["),
            parse_decimal_u64,
            tag("]"),
        )), |(_, address, _)| address),
        tag(" = "), 
        parse_decimal_u64,
    )(i)
}

fn prepare_input(input: &str) -> PreparedInput {
    all_consuming(many1(
        separated_pair(
            parse_mask,
            newline,
            many1(terminated(
                parse_instruction,
                opt(newline)
            ))
        )
    ))(input.as_bytes()).expect("").1
}

fn part1(input: &PreparedInput) -> u64 {
    let memory: HashMap<u64, u64> = input.iter().fold(HashMap::new(), |memory, ((zeros, ones), instructions)| {
        instructions.iter().fold(memory, |mut memory, &(address, value)| {
            memory.insert(address, (value | ones) & !zeros);
            memory
        })
    });

    memory.values().sum()
}

fn part2(input: &PreparedInput) -> u64 {
    let memory: HashMap<u64, u64> = input.iter().fold(HashMap::new(), |memory, ((zeros, ones), instructions)| {
        let not_x: u64 = zeros|ones;
        let mut offsets: Vec<u64> = vec![0];

        for i in 0..36 {
            let considering: u64 = 1 << i;
            let is_x = (not_x ^ considering) & considering == considering;
            if is_x {
                let len = offsets.len();
                for j in 0..len {
                    offsets.push(offsets[j] | considering);
                }
            }
        }

        instructions.iter().fold(memory, |mut memory, &(address, value)| {
            let masked: u64 = (address & not_x) | ones;

            offsets.iter().for_each(|&offset| {
                memory.insert(masked + offset, value);
            });

            memory
        })
    });

    memory.values().sum()
}

fn main() {
    let input = fs::read_to_string("./src/day14/input.txt").expect("Unable to read input file");

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
"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";

#[test]
    fn will_parse_mask() {
        assert_eq!(parse_mask(r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".as_bytes()), Ok((
            r"".as_bytes(),
            (2, 64),
        )));
    }
    #[test]
        fn will_parse_instruction() {
            assert_eq!(parse_instruction(r"mem[8] = 11".as_bytes()), Ok((
                r"".as_bytes(),
                (8, 11),
            )));
        }

    #[test]
    fn will_parse_example() {
        assert_eq!(prepare_input(EXAMPLE_INPUT), vec![(
            (
                2,
                64,
            ),
            vec![
                (8, 11),
                (7, 101),
                (8, 0),
            ],
        )]);
    }
    
    #[test]
    fn part1_example() {
        assert_eq!(part1(&prepare_input(EXAMPLE_INPUT)), 165);
    }
    
    #[test]
    fn part2_example() {
        assert_eq!(part2(&prepare_input(
"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
        )), 208);
    }
}