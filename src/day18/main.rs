#![feature(test)]

use std::fs;
use aoc_2020_rust::util::{bench, parser::parse_decimal_u64};
use nom::IResult;
use nom::sequence::*;
use nom::character::complete::*;
use nom::branch::*;
use nom::bytes::complete::*;
use nom::combinator::*;
use nom::multi::*;

#[derive(Debug, Clone)]
enum Operation {
    ADDITION,
    MULTIPLICATION,
}

fn parse_expression1(i: &[u8]) -> IResult<&[u8], u64> {
    fn parse_single_expression(i: &[u8]) -> IResult<&[u8], u64> {
        alt((
            delimited(char('('), parse_expression1, char(')')),
            parse_decimal_u64
        ))(i)
    }

    let (i, a) = parse_single_expression(i)?;

    fold_many0(
        tuple((
            delimited(
                char(' '), 
                alt((
                    value(Operation::ADDITION, char('+')),
                    value(Operation::MULTIPLICATION, char('*'))
                )), 
                char(' ')
            ),
            parse_single_expression
        )),
        a,
        |aggregate, (operation, value)| {
            match operation {
                Operation::ADDITION => aggregate + value,
                Operation::MULTIPLICATION => aggregate * value,
            }
        }
    )(i)
}

fn part1(input: &str) -> u64 {
    input.lines().map(|line| {
        parse_expression1(line.as_bytes()).unwrap().1
    }).sum()
}

fn parse_expression2(i: &[u8]) -> IResult<&[u8], u64> {
    fn parse_inner_expression(i: &[u8]) -> IResult<&[u8], u64> {
        fn parse_single_expression(i: &[u8]) -> IResult<&[u8], u64> {
            alt((
                delimited(char('('), parse_expression2, char(')')),
                parse_decimal_u64
            ))(i)
        }

        let (i, a) = parse_single_expression(i)?;
        fold_many0(
            preceded(
                tag(" + "),
                parse_single_expression
            ), 
            a, 
            |a, b| {
                a + b
            }
        )(i)
    }
    
    let (i, a) = parse_inner_expression(i)?;
    fold_many0(
        preceded(
            tag(" * "),
            parse_inner_expression
        ), a, |a, b| {
        a * b
    })(i)
}

fn part2(input: &str) -> u64 {
    input.lines().map(|line| {
        parse_expression2(line.as_bytes()).unwrap().1
    }).sum()
}

fn main() {
    let input = fs::read_to_string("./src/day18/input.txt").expect("Unable to read input file");

    let part1 = bench::time("part 1", || part1(&input));
    let part2 = bench::time("part 2", || part2(&input));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1_evaluate() {
        fn evaluate1(input: &str) -> u64 {
            parse_expression1(input.as_bytes()).unwrap().1
        }
        assert_eq!(evaluate1("1"), 1);
        assert_eq!(evaluate1("(1)"), 1);
        assert_eq!(evaluate1("2 + 3"), 5);
        assert_eq!(evaluate1("2 * 3"), 6);

        assert_eq!(evaluate1("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(evaluate1("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(evaluate1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(evaluate1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(evaluate1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(evaluate1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
    }
    
    #[test]
    fn part2_evaluate() {
        fn evaluate2(input: &str) -> u64 {
            parse_expression2(input.as_bytes()).unwrap().1
        }
        assert_eq!(evaluate2("1"), 1);
        assert_eq!(evaluate2("(1)"), 1);
        assert_eq!(evaluate2("2 + 3"), 5);
        assert_eq!(evaluate2("2 * 3"), 6);

        assert_eq!(evaluate2("1 + 2 * 3 + 4 * 5 + 6"), 231);
        assert_eq!(evaluate2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(evaluate2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(evaluate2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(evaluate2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
        assert_eq!(evaluate2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
    }
}