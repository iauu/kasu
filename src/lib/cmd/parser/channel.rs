use nom::branch::alt;
use nom::bytes::tag;
use nom::combinator::{opt, recognize};
use nom::{IResult, Parser};
use nom::character::char;
use nom::character::complete::alphanumeric1;
use nom::sequence::{delimited, pair, preceded};
use slack_morphism::{SlackChannelId};
use crate::lib::cmd::parse::Parse;
use crate::lib::cmd::parser::string::word;

pub fn parse_channel_id(arg: &str) -> IResult<&str, SlackChannelId> {
    let (remainder, result) = recognize(
        pair(
            alt((char('C'), char('D'))),
            alphanumeric1
        )
    ).parse(arg)?;
    Ok((remainder, SlackChannelId::from(result)))
}

impl Parse for SlackChannelId {
    fn parse(arg: &str) -> IResult<&str, Self> {
        alt((
            delimited(tag("<#"), parse_channel_id, preceded(opt((tag("|"), word)), tag(">"))),
            parse_channel_id
        )).parse(arg)
    }
}