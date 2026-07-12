use nom::IResult;
use nom::number::complete::recognize_float;
use slack_morphism::SlackTs;
use crate::lib::cmd::parse::Parse;

pub fn parse_ts(s: &str) -> IResult<&str, SlackTs> {
    let (remainder, result) = recognize_float(s)?;
    Ok((remainder, SlackTs::from(result)))
}

impl Parse for SlackTs {
    fn parse(arg: &str) -> IResult<&str, Self> {
        parse_ts(arg)
    }
}