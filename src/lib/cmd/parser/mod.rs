use nom::IResult;
use crate::lib::cmd::parse::Parse;

mod quoted;
mod string;
mod user;
mod channel;
mod ts;

impl<X: Parse> Parse for Option<X> {
    fn parse(arg: &str) -> IResult<&str, Self> {
        let res = X::parse(arg);
        Ok(match res {
            Ok(r) => (r.0, Some(r.1)),
            Err(_) => (arg, None)
        })
    }
}