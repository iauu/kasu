use nom::branch::alt;
use nom::bytes::tag;
use nom::combinator::{opt, recognize};
use nom::{IResult, Parser};
use nom::character::char;
use nom::character::complete::alphanumeric1;
use nom::sequence::{delimited, pair, preceded};
use slack_morphism::SlackUserId;
use crate::lib::cmd::parse::Parse;
use crate::lib::cmd::parser::string::word;

pub fn parse_user_id(arg: &str) -> IResult<&str, SlackUserId> {
    let (remainder, result) = recognize(
        pair(
            alt((char('U'), char('W'), char('B'))),
            alphanumeric1
        )
    ).parse(arg)?;
    Ok((remainder, SlackUserId::from(result)))
}

impl Parse for SlackUserId {
    fn parse(arg: &str) -> IResult<&str, Self> {
        alt((
            delimited(tag("<@"), parse_user_id, preceded(opt((tag("|"), word)), tag(">"))),
            parse_user_id
        )).parse(arg)
    }
}