
use nom::{IResult, bytes::complete::take_while1, character::is_digit, combinator::map};

pub fn parse_decimal_u32(i: &[u8]) -> IResult<&[u8], u32> {
    map(take_while1(is_digit), |digits: &[u8]| {
        digits.iter().fold(0, |number, c| {
            number * 10 + (*c - b'0') as u32
        })
    })(i)
}

pub fn parse_decimal_u64(i: &[u8]) -> IResult<&[u8], u64> {
    map(take_while1(is_digit), |digits: &[u8]| {
        digits.iter().fold(0, |number, c| {
            number * 10 + (*c - b'0') as u64
        })
    })(i)
}