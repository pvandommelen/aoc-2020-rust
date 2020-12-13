use std::fs;
use core::fmt::Debug;
use aoc_2020_rust::util::{bench, parser::parse_decimal_u32};
use nom::{IResult, branch::alt, bytes::complete::tag, character::complete::newline, combinator::{all_consuming, map, opt}, multi::many1, sequence::{pair, terminated}};

#[derive(Debug, PartialEq, Copy, Clone)]
enum InstructionKind {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    LEFT,
    RIGHT,
    FORWARD,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Instruction {
    kind: InstructionKind,
    amount: i32,
}

fn parse_instruction(i: &[u8]) -> IResult<&[u8], Instruction> {
    map(
        pair(
            alt((
                map(tag("N"), |_| InstructionKind::NORTH),
                map(tag("S"), |_| InstructionKind::SOUTH),
                map(tag("E"), |_| InstructionKind::EAST),
                map(tag("W"), |_| InstructionKind::WEST),
                map(tag("L"), |_| InstructionKind::LEFT),
                map(tag("R"), |_| InstructionKind::RIGHT),
                map(tag("F"), |_| InstructionKind::FORWARD),
            )),
            parse_decimal_u32
        ),
        |(kind, number)| -> Instruction {
            Instruction {
                kind,
                amount: number as i32
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

fn part1(input: &PreparedInput) -> i32 {
    struct State {
        x: i32,
        y: i32,
        direction: i32,
    }
    
    fn apply(state: &mut State, instruction: &Instruction) {
        match instruction.kind {
            InstructionKind::EAST => { state.x += instruction.amount }
            InstructionKind::WEST => { state.x -= instruction.amount }
            InstructionKind::NORTH => { state.y += instruction.amount }
            InstructionKind::SOUTH => { state.y -= instruction.amount }
            InstructionKind::LEFT => { state.direction += instruction.amount }
            InstructionKind::RIGHT => { state.direction -= instruction.amount }
            InstructionKind::FORWARD => {
                let normalized = state.direction % 360 + if state.direction % 360 < 0 { 360 } else { 0 };
    
                match normalized {
                    0 => state.x += instruction.amount,
                    90 => state.y += instruction.amount,
                    180 => state.x -= instruction.amount,
                    270 => state.y -= instruction.amount,
                    _ => panic!("Direction not exactly at 90 degree angle {}", normalized),
                }
            }
        };
    }

    let mut state = State {
        x: 0,
        y: 0,
        direction: 0,
    };

    input.iter().for_each(|instruction| {
        apply(&mut state, instruction);
    });

    state.x.abs() + state.y.abs()
}

fn part2(input: &PreparedInput) -> i32 {

    #[derive(Debug)]
    struct State {
        x: i32,
        y: i32,
        wp_x: i32,
        wp_y: i32,
    }
    
    fn apply(state: &mut State, instruction: &Instruction) {
        match instruction.kind {
            InstructionKind::EAST => { state.wp_x += instruction.amount }
            InstructionKind::WEST => { state.wp_x -= instruction.amount }
            InstructionKind::NORTH => { state.wp_y += instruction.amount }
            InstructionKind::SOUTH => { state.wp_y -= instruction.amount }
            InstructionKind::LEFT => {
                for _ in 0..(instruction.amount / 90) {
                    // 0 -1
                    // 1  0
                    let (wp_x, wp_y) = (state.wp_x, state.wp_y);
                    state.wp_x = -wp_y;
                    state.wp_y = wp_x;
                }
            }
            InstructionKind::RIGHT => {
                for _ in 0..(instruction.amount / 90) {
                    // 0  1
                    // -1 0
                    let (wp_x, wp_y) = (state.wp_x, state.wp_y);
                    state.wp_x = wp_y;
                    state.wp_y = -wp_x;
                }
            }
            InstructionKind::FORWARD => {
                state.x += state.wp_x * instruction.amount;
                state.y += state.wp_y * instruction.amount;
            }
        };
    }

    let mut state = State {
        x: 0,
        y: 0,
        wp_x: 10,
        wp_y: 1,
    };

    input.iter().for_each(|instruction| {
        apply(&mut state, instruction);
    });

    state.x.abs() + state.y.abs()
}

fn main() {
    let input = fs::read_to_string("./src/day12/input.txt").expect("Unable to read input file");

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
            parse_instruction(r"F10".as_bytes()), 
            Ok((
                r"".as_bytes(), 
                Instruction { kind: InstructionKind::FORWARD, amount: 10 }
            ))
        );
        assert_eq!(
            parse_instruction(r"R90".as_bytes()), 
            Ok((
                r"".as_bytes(), 
                Instruction { kind: InstructionKind::RIGHT, amount: 90 }
            ))
        );
    }

    const EXAMPLE_INPUT: &str = 
"F10
N3
F7
R90
F11";
    
    #[test]
    fn part1_example() {
        assert_eq!(part1(&prepare_input(EXAMPLE_INPUT)), 25);
    }
    
    #[test]
    fn part2_example() {
        assert_eq!(part2(&prepare_input(EXAMPLE_INPUT)), 286);
    }
}