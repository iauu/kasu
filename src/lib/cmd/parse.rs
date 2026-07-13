use async_trait::async_trait;
use nom::{IResult};
use crate::lib::cmd::CmdEvent;
use crate::lib::cmd::event::CmdParsedEvent;
use crate::lib::context::AsyncSafe;
use crate::lib::transform::Transform;

pub trait CmdParse<Args = ()> : Send + Sync + Clone + 'static {
    fn parse<'a>(arg_raw: &'a str) -> IResult<&'a str, Args>;
}

#[async_trait::async_trait]
pub trait CmdTransformParse<Args = ()> : Send + Sync + Clone + 'static {
    async fn transform_parse(arg_raw: &str) -> IResult<&str, Args>;
}

pub trait Parse : Sized + Send + Sync + 'static { //  : + Clone + 'static
    fn parse(arg: &str) -> IResult<&str, Self>;
}

impl CmdParse<()> for CmdEvent {
    fn parse<'a>(arg_raw: &'a str) -> IResult<&'a str, ()> {
        Ok((arg_raw, ()))
    }
}

#[async_trait::async_trait]
impl<Args, T: CmdParse<Args>> CmdTransformParse<Args> for T {
    async fn transform_parse(arg_raw: &str) -> IResult<&str, Args> {
        Self::parse(arg_raw)
    }
}

pub trait ToParsed<Args> {
    fn to_parsed(event: CmdEvent) -> Option<CmdParsedEvent<Args>>;
}

#[async_trait::async_trait]
pub trait TransformParse : Sized + Send + Sync {
    async fn transform_parse(arg: &str) -> IResult<&str, Self>;
}

#[async_trait::async_trait]
impl<T> TransformParse for T
where T: Parse {
    async fn transform_parse(arg: &str) -> IResult<&str, Self> {
        Self::parse(arg)
    }
}

impl<Args, T> ToParsed<Args> for T
where T: CmdParse<Args> {
    fn to_parsed(event: CmdEvent) -> Option<CmdParsedEvent<Args>> {
        let (arg_raw, arg) = match T::parse(event.arg_raw.as_str()) {
            Ok(e) => e,
            Err(_) => return None
        };
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

#[async_trait::async_trait]
pub trait ToTransformParsed<Args> {
    async fn to_transform_parsed(event: CmdEvent) -> Option<CmdParsedEvent<Args>>;
}

#[async_trait::async_trait]
impl<Args, T> ToTransformParsed<Args> for T
where T: CmdTransformParse<Args> {
    async fn to_transform_parsed(event: CmdEvent) -> Option<CmdParsedEvent<Args>> {
        let (arg_raw, arg) = match T::transform_parse(event.arg_raw.as_str()).await {
            Ok(e) => e,
            Err(_) => return None
        };
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
            impl <$($arg_name,)*> CmdTransformParse<($($arg_name,)*)> for $crate::lib::cmd::CmdEvent
            where
            $(
                $arg_name: $crate::lib::cmd::parse::TransformParse,
            )*
            {
                async fn transform_parse(arg_raw: &str) -> ::nom::IResult<&str, ($($arg_name,)* )> {
                    use ::nom::character::complete::multispace1;
                    use ::nom::sequence::preceded;
                    use ::nom::Parser;
                    // ($(preceded(multispace1, $arg_name::parse),)*).parse(&arg_raw)
                    $(
                        let (arg_raw, _) = multispace1(arg_raw)?;
                        let (arg_raw, [< $arg_name _a >]) = $arg_name::transform_parse(arg_raw).await?;
                    )*
                    Ok((arg_raw, ($([< $arg_name _a >], )*)))
                }
            }

            #[::async_trait::async_trait]
            impl <$($arg_name,)*> $crate::lib::event::TransformFromEvent for $crate::lib::cmd::event::CmdParsedEvent<($($arg_name,)*)>
            where
            $(
                $arg_name: $crate::lib::cmd::parse::TransformParse + $crate::lib::context::AsyncSafe,
            )* {
                async fn transform_from_event(event: $crate::lib::event::Event) -> Option<Self> {
                    use $crate::lib::event::FromEvent;
                    let event: CmdEvent = CmdEvent::from_event(event)?;
                    CmdEvent::to_transform_parsed(event).await
                }
            }

            impl <$($arg_name,)*> $crate::lib::cmd::event::TransFromEventCmd for $crate::lib::cmd::event::CmdParsedEvent<($($arg_name,)*)>
            where
            $(
                $arg_name: $crate::lib::cmd::parse::TransformParse + $crate::lib::context::AsyncSafe,
            )* {}
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