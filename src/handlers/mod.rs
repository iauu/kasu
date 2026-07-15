pub(crate) mod test_msg_listen;
pub(crate) mod msg_respond;
pub(crate) mod bot_msg_send;
pub(crate) mod get_user_id;
pub(crate) mod init;
pub(crate) mod user_join_event;

/// Log the event and ignore failure. Error store in `e` and result store in `res`
#[macro_export]
macro_rules! fail_ignore_handle {
    ($event:expr, $lit_err:literal) => {
        let res = $event;
        if let Err(e) = res {
            tracing::error!($lit_err, e = e);
        }
    };
    ($event:expr, $lit_ok:literal, $lit_err:literal) => {
        match $event {
            Ok(res)  => {
                tracing::info!($lit_ok, res = res);
            },
            Err(e) => {
                tracing::error!($lit_err, e = e);
            }
        }
    }
}

/// Log the event and ignore failure. Error store in `e` and result store in `res`. Return the expression if error
#[macro_export]
macro_rules! fail_ret_handle {
    ($event:expr, $lit_err:literal, $err:expr) => {
        let res = $event;
        if let Err(e) = res {
            tracing::error!($lit_err, e = e);
            return $err;
        }
        res
    };
    ($event:expr, $lit_ok:literal, $lit_err:literal) => {
        match $event {
            Ok(res)  => {
                tracing::info!($lit_ok, res = res);
                res
            },
            Err(e) => {
                tracing::error!($lit_err, e = e);
                return $err;
            }
        }
    }
}