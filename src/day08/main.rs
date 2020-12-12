use std::{collections::HashSet, fs};
use core::fmt::Debug;
use aoc_2020_rust::util::bench;
use nom::{IResult, branch::alt, bytes::complete::tag, character::complete::{digit1, newline, one_of}, combinator::{all_consuming, map, opt}, multi::many1, sequence::{separated_pair, terminated, tuple}};

#[derive(Debug, PartialEq, Copy, Clone)]
enum InstructionKind {
    NOP,
    ACC,
    JMP,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Instruction {
    kind: InstructionKind,
    amount: i32,
}

fn parse_decimal_number(i: &[u8]) -> IResult<&[u8], i32> {
    map(digit1, |digits: &[u8]| std::str::from_utf8(digits).unwrap().parse().unwrap())(i)
}

fn parse_instruction(i: &[u8]) -> IResult<&[u8], Instruction> {
    map(
        separated_pair(
            alt((
                map(tag("nop"), |_| InstructionKind::NOP),
                map(tag("acc"), |_| InstructionKind::ACC),
                map(tag("jmp"), |_| InstructionKind::JMP),
            )),
            tag(" "),
            map(
                tuple((
                    one_of("+-"),
                    parse_decimal_number,
                )), 
                |(plus_or_minus, number)| {
                    let sign = if plus_or_minus == '-' { -1 } else { 1 };
                    sign * number
                }
            )
        ),
        |(kind, number)| -> Instruction {
            Instruction {
                kind: kind,
                amount: number
            }
        }
    )(i)
}

type PreparedInput = Vec<Instruction>;

fn prepare_input(input: &str) -> PreparedInput {
    all_consuming(many1(
        terminated(
            parse_instruction,
            opt(newline)
        )
    ))(input.as_bytes()).expect("").1
}

struct State {
    index: usize,
    aggregate: i32,
}

fn apply(state: &mut State, instruction: &Instruction) {
    match instruction.kind {
        InstructionKind::ACC => state.aggregate += instruction.amount,
        _ => {}
    };
    
    state.index = match instruction.kind {
        InstructionKind::JMP => (state.index as i32 + instruction.amount) as usize,
        _ => state.index + 1
    };
}

fn run(input: &PreparedInput) -> (i32, bool) {
    let mut visited = HashSet::new();
    let mut state = State {
        index: 0,
        aggregate: 0,
    };

    while state.index < input.len() && !visited.contains(&state.index) {
        visited.insert(state.index);

        let instruction = input.get(state.index).unwrap();

        apply(&mut state, instruction);
    }

    (state.aggregate, state.index >= input.len())
}

fn part1(input: &PreparedInput) -> i32 {
    let result = run(input);
    assert!(result.1 == false);
    result.0
}

fn part2(input: &PreparedInput) -> i32 {
    let flippable = input.iter().enumerate().filter(|(_, instruction)| {
        match instruction.kind {
            InstructionKind::JMP => true,
            InstructionKind::NOP => true,
            _ => false,
        }
    });

    flippable.map(|(index, instruction)| {
        let mut copy = input.to_owned();
        let flipped_instruction = match instruction.kind {
            InstructionKind::NOP => Instruction { kind: InstructionKind::JMP, amount: instruction.amount },
            InstructionKind::JMP => Instruction { kind: InstructionKind::NOP, amount: instruction.amount },
            _ => panic!(),
        };
        copy[index] = flipped_instruction;
        run(&copy)
    }).find(|(_, success)| {
        *success
    }).unwrap().0
}

fn main() {
    let input = fs::read_to_string("./src/day08/input.txt").expect("Unable to read input file");

    let prepared_input = bench::time("input preparation", || prepare_input(&input));
    let part1 = bench::time("part 1", || part1(&prepared_input));
    let part2 = bench::time("part 2", || part2(&prepared_input));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn will_parse_instruction() {
        assert_eq!(
            parse_instruction(r"nop +0".as_bytes()), 
            Ok((
                r"".as_bytes(), 
                Instruction { kind: InstructionKind::NOP, amount: 0 }
            ))
        );
        assert_eq!(
            parse_instruction(r"acc +1".as_bytes()), 
            Ok((
                r"".as_bytes(), 
                Instruction { kind: InstructionKind::ACC, amount: 1 }
            ))
        );
        assert_eq!(
            parse_instruction(r"acc -11".as_bytes()), 
            Ok((
                r"".as_bytes(), 
                Instruction { kind: InstructionKind::ACC, amount: -11 }
            ))
        );
        assert_eq!(
            parse_instruction(r"jmp +1".as_bytes()), 
            Ok((
                r"".as_bytes(), 
                Instruction { kind: InstructionKind::JMP, amount: 1 }
            ))
        );
    }

    const EXAMPLE_INPUT: &str = 
"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    
    #[test]
    fn part1_example() {
        assert_eq!(part1(&prepare_input(EXAMPLE_INPUT)), 5);
    }
    
    #[test]
    fn part2_example() {
        assert_eq!(part2(&prepare_input(EXAMPLE_INPUT)), 8);
    }
    
    #[test]
    fn part2_flip_jump() {
        assert_eq!(part2(&prepare_input(
"jmp +0"
        )), 0);
    }
}