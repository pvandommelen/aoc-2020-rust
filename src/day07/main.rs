#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{collections::{HashMap, HashSet}, fs, str};
use aoc_2020_rust::util::bench;

#[macro_use]
extern crate nom;
extern crate regex;

use nom::{IResult, bytes::complete::{tag, take_while}, character::is_alphabetic, character::{complete::{digit0, digit1, newline}, is_digit, is_space}, error::ParseError, multi::many1, number::complete::u8, regexp::bytes::{re_find, re_match}, sequence::preceded};
use nom::combinator::{map, opt};
use nom::branch::alt;
#[derive(Debug, PartialEq)]
pub struct BagStatement<'a> {
    pub color: &'a [u8],
    pub contents: Vec<(&'a [u8], u8)>,
}

fn parse_color(input: &[u8]) -> nom::IResult<&[u8], &[u8]> {
    let re = regex::bytes::Regex::new(r"^[a-z]+ [a-z]+").unwrap();
    re_find(re)(input)
}

fn parse_amount(i: &[u8]) -> IResult<&[u8], u8> {
    map(digit1, |digits: &[u8]| str::from_utf8(digits).unwrap().parse().unwrap())(i)
}

fn parse_bag_content_statement(i: &[u8]) -> nom::IResult<&[u8], (&[u8], u8)> {
    let (i, amount) = parse_amount(i)?;
    let (i, _) = take_while(is_space)(i)?;
    let (i, color) = parse_color(i)?;
    let (i, _) = tag(" bag")(i)?;
    let (i, _) = opt(tag("s"))(i)?;
    Ok((i, (color, amount)))
}

fn parse_bag_statement(i: &[u8]) -> nom::IResult<&[u8], BagStatement> {
    let (i, color) = parse_color(i)?;
    let (i, _) = tag(" bags contain ")(i)?;
    let (i, contents ) = alt((
        map(tag("no other bags"), |_| vec![]),
        many1(preceded(opt(tag(", ")), map(
            parse_bag_content_statement,
            |(color, amount)| (color.into(), amount)
        )))
    ))(i)?;

    let (i, _) = tag(".")(i)?;
    let (i, _) = opt(newline)(i)?;

    Ok((i, BagStatement {
        color: color.into(),
        contents,
    }))
}

type PreparedInput<'a> = Vec<BagStatement<'a>>;

fn prepare_input(input: &str) -> PreparedInput {
    let i = input.as_bytes();

    let (i, bag_statements) = many1(parse_bag_statement)(i).unwrap();
    assert!(i.len() == 0, "input remaining {:?}", str::from_utf8(i));
    bag_statements
}

fn part1<'a>(input: &'a PreparedInput) -> u32 {
    let mut stack: Vec<&'a [u8]> = [ "shiny gold".as_bytes().into() ].iter().cloned().collect();
    let mut full: HashSet<&'a [u8]> = HashSet::new();

    while let Some(color)=stack.pop() {
        // look for all bags that contain this color
        input.iter().for_each(|bag_statement| {
            bag_statement.contents.iter().for_each(|(contains_color, amount)| {
                if **contains_color == *color {
                    if !full.contains(bag_statement.color) {
                        full.insert(bag_statement.color);
                        stack.push(bag_statement.color);
                    }
                }
            });
        });
    }
    full.len() as u32
}
fn part2(input: &PreparedInput) -> u32 {
    0
}

fn main() {
    let input = fs::read_to_string("./src/day07/input.txt").expect("Unable to read input file");

    let prepared_input = bench::time("input preparation", || prepare_input(&input));
    let part1 = bench::time("part 1", || part1(&prepared_input));
    let part2 = bench::time("part 2", || part2(&prepared_input));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;

    use super::*;

    #[test]
    fn will_parse_color() {
        assert_eq!(parse_color("light red bags".as_bytes()), Ok((r" bags".as_bytes(), "light red".as_bytes())));
    }

    #[test]
    fn will_parse_amount() {
        assert_eq!(
            parse_amount(r"0".as_bytes()), 
            Ok((
                r"".as_bytes(), 
                0
            ))
        );
        assert_eq!(
            parse_amount(r"123".as_bytes()), 
            Ok((
                r"".as_bytes(), 
                123
            ))
        );
    }

    #[test]
    fn will_parse_bag_content_statement() {
        assert_eq!(
            parse_bag_content_statement(r"1 bright white bag".as_bytes()), 
            Ok((
                r"".as_bytes(), 
                (r"bright white".as_bytes(), 1)
            ))
        );
    }

    #[test]
    fn will_parse_bag_statement() {
        assert_eq!(
            parse_bag_statement(r"light red bags contain 1 bright white bag, 2 muted yellow bags.".as_bytes()), 
            Ok((
                r"".as_bytes(), 
                BagStatement {
                    color: r"light red".as_bytes().into(),
                    contents: vec![
                        (r"bright white".as_bytes().into(), 1),
                        (r"muted yellow".as_bytes().into(), 2),
                    ]
                }
            ))
        );

        assert_eq!(
            parse_bag_statement(r"light red bags contain no other bags.".as_bytes()), 
            Ok((
                r"".as_bytes(), 
                BagStatement {
                    color: r"light red".as_bytes().into(),
                    contents: vec![]
                }
            ))
        );
    }

    const EXAMPLE_INPUT: &str = 
"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    
    #[test]
    fn will_count_part1_example() {
        assert_eq!(part1(&prepare_input(EXAMPLE_INPUT)), 4);
    }
    
    #[test]
    fn will_count_part2_example() {
        assert_eq!(part2(&prepare_input(EXAMPLE_INPUT)), 0);
    }
}