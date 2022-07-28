use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::map_res,
    error::{Error, ErrorKind, ParseError},
    sequence::separated_pair,
    Err, IResult,
};

use super::{Number, Pair};

fn brackets(left: char, right: char) -> impl Fn(&str) -> IResult<&str, &str> {
    move |input: &str| {
        let (input, _) = char('[')(input)?;
        let mut counter: usize = 1;
        for (i, c) in input.chars().enumerate() {
            if c == right {
                counter -= 1;
                if counter == 0 {
                    return Ok((&input[i + 1..], &input[0..i]));
                }
            } else if c == left {
                counter += 1;
            }
        }
        Err(Err::Error(Error::from_error_kind(
            input,
            ErrorKind::TakeUntil,
        )))
    }
}

fn num_num(input: &str) -> IResult<&str, Number> {
    let (input, n) = map_res(digit1, str::parse::<u8>)(input)?;
    IResult::Ok((input, Number::Number(n)))
}

fn num_pair(input: &str) -> IResult<&str, Number> {
    let (input, pair) = pair(input)?;
    IResult::Ok((input, Number::Pair(pair)))
}

fn number(input: &str) -> IResult<&str, Number> {
    alt((num_pair, num_num))(input)
}

pub fn pair(input: &str) -> IResult<&str, Pair> {
    let (input, inner) = brackets('[', ']')(input)?;
    let (_, (left, right)) = separated_pair(number, char(','), number)(inner)?;
    IResult::Ok((
        input,
        Pair {
            left: Box::new(left),
            right: Box::new(right),
        },
    ))
}
