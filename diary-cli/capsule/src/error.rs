use std::{fmt, error::Error};
pub use crate::gen_error_handler;
pub use crate::handle;

#[derive(Debug)]
pub enum CapErrContext<'a> {
    WhileZippingFile(&'a str),
    WhileBuildingTarBall(&'a str),
    Undefined,
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

pub trait CapErrHandler<'a> {
    /// Sets the context of the capsule error
    fn set_context(&mut self, context: &'a impl Fn() -> CapErrContext<'a>);

    /// Gets the context of the capsule error
    fn get_context(&self) -> CapErrContext<'a>;

    /// Initialises handler object for easier passing between functions
    fn init() -> Self;

    /// Run at runtime when an error occurs rather than returning the error (for performance)
    fn runtime<T>(&self, error: CapError, context: CapErrContext, retry: Option<&dyn Fn() -> Result<T, CapError>>) -> T;
}

#[macro_export]
macro_rules! gen_error_handler {
    ($($pattern:pat => $result:expr),* $(,)?) => {{
        pub struct CapErrHandler<'a> { context: &'a dyn Fn() -> CapErrContext<'a> }
        impl<'a> Clone for CapErrHandler<'a> { fn clone(&self) -> Self { Self::init() } }
        impl<'a> $crate::error::CapErrHandler<'a> for CapErrHandler<'a> {
            fn set_context(&mut self, context: &'a impl Fn() -> CapErrContext<'a>) { self.context = context; }
            fn get_context(&self) -> CapErrContext<'a> { (*self.context)() }
            fn init() -> Self { Self { context: &|| $crate::error::CapErrContext::Undefined } }
            fn runtime<T>(&self, error: $crate::error::CapError, context: CapErrContext, retry: Option<&dyn Fn() -> Result<T, $crate::error::CapError>>) -> T {
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
    (($handler:ident) ($action:expr) => $result:expr) => {{
        let res: Result<_, _> = $action;
        if let Err(e) = res {
            $handler.runtime($result(e), $handler.get_context(), None)
        } else { res.unwrap() }
    }};

    // For use on results with other errors
    (($handler:ident) $action:expr => $result:expr) => {{
        let res: Result<_, _> = $action;
        if let Err(e) = res {
            $handler.runtime($result(e), $handler.get_context(), Some(&|| {
                let res = $action;
                if let Err(e) = res {
                    Err($result(e))
                } else { Ok(res.unwrap()) }
            }))
        } else { res.unwrap() }
    }};

    // For use on results with cap errors (once)
    (($handler:ident) ($action:expr)) => {{
        let res: Result<_, $crate::error::CapError> = $action;
        if let Err(e) = res {
            $handler.runtime(e, $handler.get_context(), None)
        } else { res.unwrap() }
    }};

    // For use on results with cap errors
    (($handler:ident) $action:expr) => {{
        let res: Result<_, $crate::error::CapError> = $action;
        if let Err(e) = res {
            $handler.runtime(e, $handler.get_context(), Some(&|| $action))
        } else { res.unwrap() }
    }};
}