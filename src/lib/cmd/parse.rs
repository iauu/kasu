use nom::{IResult};
use crate::lib::cmd::CmdEvent;
use crate::lib::cmd::event::CmdParsedEvent;

pub trait CmdParse<Args = ()> : Send + Sync + Clone + 'static {
    fn parse<'a>(arg_raw: &'a str) -> IResult<&'a str, Args>;
}

pub trait Parse : Sized + Send + Sync + 'static { //  : + Clone + 'static
    fn parse(arg: &str) -> IResult<&str, Self>;
}

impl CmdParse<()> for CmdEvent {
    fn parse<'a>(arg_raw: &'a str) -> IResult<&'a str, ()> {
        Ok((arg_raw, ()))
    }
}


pub trait ToParsed<Args> {
    fn to_parsed(event: CmdEvent) -> Option<CmdParsedEvent<Args>>;
}

impl<Args, T> ToParsed<Args> for T
where T: CmdParse<Args> {
    fn to_parsed(event: CmdEvent) -> Option<CmdParsedEvent<Args>> {
        let (arg_raw, arg) = T::parse(event.arg_raw.as_str()).unwrap();
        if arg_raw.len() > 0 {
            return None;
        }
        Some(CmdParsedEvent {
            command: event.command,
            arg_raw: event.arg_raw,
            user_id: event.user_id,
            channel_id: event.channel_id,
            thread_ts: event.thread_ts,
            message_ts: event.message_ts,
            arg
        })
    }
}

macro_rules! impl_cmd_parse {
    ($($arg_name:ident)*) => {
        ::paste::paste!{
            #[allow(non_snake_case, non_camel_case_types, unused)]
            #[::async_trait::async_trait]
            impl <$($arg_name,)*> CmdParse<($($arg_name,)*)> for $crate::lib::cmd::CmdEvent
            where
            $(
                $arg_name: $crate::lib::cmd::parse::Parse,
            )*
            {
                fn parse<'a>(arg_raw: &'a str) -> ::nom::IResult<&'a str, ($($arg_name,)* )> {
                    use ::nom::character::complete::multispace1;
                    use ::nom::sequence::preceded;
                    use ::nom::Parser;
                    ($(preceded(multispace1, $arg_name::parse),)*).parse(&arg_raw)
                }
            }

            impl <$($arg_name,)*> $crate::lib::event::FromEvent for $crate::lib::cmd::event::CmdParsedEvent<($($arg_name,)*)>
            where
            $(
                $arg_name: $crate::lib::cmd::parse::Parse,
            )* {
                fn from_event(event: $crate::lib::event::Event) -> Option<Self> {
                    let event: CmdEvent = CmdEvent::from_event(event)?;
                    CmdEvent::to_parsed(event)
                }
            }
        }
    };
}

macro_rules! multi_impl_cmd_parse {
    () => {};

    ($head:ident $($tail:ident)*) => {
        multi_impl_cmd_parse!($($tail)*);

        impl_cmd_parse!($head $($tail)*);
    };

}

multi_impl_cmd_parse!(A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14 A15);