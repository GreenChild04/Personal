use std::{fmt, error::Error};
pub use crate::gen_error_handler;
pub use crate::handle;

#[derive(Debug)]
pub enum CapErrContext<'a> {
    WhileZippingFile(&'a str),
    WhileBuildingTarBall(&'a str),
}

#[derive(Debug)]
pub enum CapError {
    IOError(std::io::Error),
}

impl fmt::Display for CapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CapError::*;
        match self {
            IOError(e) => write!(f, "IO Error: {:?}", e),
        }
    }
}

impl Error for CapError {}

pub trait CapErrHandler {
    /// Initialises handler object for easier passing between functions
    fn init() -> Self;

    /// Run at runtime when an error occurs rather than returning the error (for performance)
    fn runtime<T>(&self, error: CapError, context: CapErrContext, retry: Option<&dyn Fn() -> Result<T, CapError>>) -> T;
}

#[macro_export]
macro_rules! gen_error_handler {
    ($($pattern:pat => $result:expr),* $(,)?) => {{
        pub struct CapErrHandler;
        impl Copy for CapErrHandler {}
        impl $crate::capsule::error::CapErrHandler for CapErrHandler {
            fn init() -> Self { Self }
            fn runtime<T>(&self, error: capsule::error::CapError, context: CapErrContext, retry: Option<&dyn Fn() -> Result<T, capsule::error::CapError>>) -> T {
                match (error, context, retry) {
                    $($pattern => $result),*
                }
            }
        }

        CapErrHandler::init()
    }}
}

#[macro_export]
macro_rules! handle {
    // For use on results with other errors (once)
    (($handler:ident $context:expr) ($action:expr) => $result:expr) => {{
        let res: Result<_, _> = $action;
        if let Err(e) = res {
            $handler.runtime($result(e), $context, None)
        } else { res.unwrap() }
    }};

    // For use on results with other errors
    (($handler:ident $context:expr) $action:expr => $result:expr) => {{
        let res: Result<_, _> = $action;
        if let Err(e) = res {
            $handler.runtime($result(e), $context, Some(&|| {
                let res = $action;
                if let Err(e) = res {
                    Err($result(e))
                } else { Ok(res.unwrap()) }
            }))
        } else { res.unwrap() }
    }};

    // For use on results with cap errors (once)
    (($handler:ident $context:expr) ($action:expr)) => {{
        let res: Result<_, capsule::error::CapError> = $action;
        if let Err(e) = res {
            $handler.runtime(e, $context, None)
        } else { res.unwrap() }
    }};

    // For use on results with cap errors
    (($handler:ident $context:expr) $action:expr) => {{
        let res: Result<_, capsule::error::CapError> = $action;
        if let Err(e) = res {
            $handler.runtime(e, $context, Some(&|| $action))
        } else { res.unwrap() }
    }};
}