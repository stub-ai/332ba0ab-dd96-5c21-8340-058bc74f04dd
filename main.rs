extern crate nom;

use nom::{
    IResult,
    bytes::complete::tag,
    sequence::tuple,
    character::complete::digit1,
    combinator::map_res,
    combinator::map,
    combinator::opt,
    sequence::preceded,
};

fn parse_integer(input: &str) -> IResult<&str, i32> {
    map_res(
        digit1,
        |digit_str: &str| digit_str.parse::<i32>()
    )(input)
}

fn parse_negative(input: &str) -> IResult<&str, i32> {
    map(
        preceded(tag("-"), parse_integer),
        |num: i32| -num
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    opt(parse_negative)(input)
        .unwrap_or_else(|_| parse_integer(input))
}

fn main() {
    assert_eq!(parse_number("123"), Ok(("", 123)));
    assert_eq!(parse_number("-123"), Ok(("", -123)));
}