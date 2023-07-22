use std::{fmt, error::Error};
pub use crate::gen_error_handler;
pub use crate::handle_cap_err;

#[derive(Debug)]
pub enum CapError {
    // todo!()
}

impl fmt::Display for CapError {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => todo!()
        }
    }
}

impl Error for CapError {}

pub trait CapErrHandler {
    /// Initialises handler object for easier passing between functions
    fn init() -> Self;

    /// Run at runtime when an error occurs rather than returning the error (for performance)
    fn runtime<T>(&self, error: CapError, retry: Option<&dyn Fn() -> Result<T, CapError>>) -> T;
}

#[macro_export]
macro_rules! gen_error_handler {
    ($($pattern:pat => $result:expr),* $(,)?) => {{
        pub struct CapErrHandler;
        impl $crate::capsule::error::CapErrHandler for CapErrHandler {
            fn init() -> Self { Self }
            fn runtime<T>(&self, error: $crate::capsule::error::CapError, retry: Option<&dyn Fn() -> Result<T, $crate::capsule::error::CapError>>) -> T {
                match (error, retry) {
                    $($pattern => $result),*
                }
            }
        }

        CapErrHandler::init()
    }}
}

#[macro_export]
macro_rules! handle_cap_err {
    // For use on results with other errors
    (($handler:ident) $action:expr => $result:expr) => {{
        let res = $action;
        if let Err(e) = res {
            $handler.runtime($result(e), Some(&|| $action))
        } else { res.unwrap() }
    }};

    // For use on results with cap errors
    (($handler:ident) $action:expr) => {{
        let res = $action;
        if let Err(e) = res {
            $handler.runtime(e, Some(&|| $action))
        } else { res.unwrap() }
    }};
}