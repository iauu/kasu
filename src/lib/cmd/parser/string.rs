use nom::branch::alt;
use nom::error::{ErrorKind, ParseError};
use nom::{AsChar, IResult, Input, Parser};
use crate::lib::cmd::parse::Parse;

pub fn word<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, String, E>
{
    let (remainder, result) = input.split_at_position1_complete(|item| item.is_space(), ErrorKind::AlphaNumeric)?;
    Ok((remainder, String::from(result)))
}

impl Parse for String {
    fn parse(arg: &str) -> IResult<&str, Self> {
        let (remainder, result) = alt(
             (
                 crate::lib::cmd::parser::quoted::parse_string,
                 word
             )
        ).parse(arg)?;
        Ok((remainder, result.to_string()))
    }
}