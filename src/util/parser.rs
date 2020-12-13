
use nom::{IResult, character::complete::digit1, combinator::map};

pub fn parse_decimal_u32(i: &[u8]) -> IResult<&[u8], u32> {
    map(digit1, |digits: &[u8]| std::str::from_utf8(digits).unwrap().parse().unwrap())(i)
}

pub fn parse_decimal_u64(i: &[u8]) -> IResult<&[u8], u64> {
    map(digit1, |digits: &[u8]| std::str::from_utf8(digits).unwrap().parse().unwrap())(i)
}