use std::{fmt, error::Error};
pub use crate::handler;
pub(crate) use crate::handle;

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

pub trait CapErrHandler {
    /// Initialises handler object for easier passing between functions
    fn init() -> Self where Self: Sized;

    /// Run at runtime when an error occurs rather than returning the error (for performance)
    fn runtime<T>(&self, error: CapError, context: &CapErrContext, retry: Option<&dyn Fn() -> Result<T, CapError>>) -> T;
}

pub(crate) struct ErrHandler<'a, T: CapErrHandler> {
    handler: T,
    context: CapErrContext<'a>,
}

impl<'a, T: CapErrHandler> ErrHandler<'a, T> {
    pub fn new(handler: T, context: CapErrContext<'a>) -> Self {
        Self {
            handler,
            context,
        }
    }

    #[inline]
    pub fn runtime<TT>(&self, error: CapError, context: &CapErrContext, retry: Option<&dyn Fn() -> Result<TT, CapError>>) -> TT {
        self.handler.runtime(error, context, retry)
    }

    #[inline]
    pub fn context(&self) -> &CapErrContext<'a> { &self.context }
}

#[macro_export]
macro_rules! handler {
    ($($pattern:pat => $result:expr),* $(,)?) => {{
        pub struct CapErrHandler;
        impl Copy for CapErrHandler {}
        impl Clone for CapErrHandler {
            #[inline]
            fn clone(&self) -> Self { Self::init() }
        }
        impl $crate::error::CapErrHandler for CapErrHandler {
            #[inline]
            fn init() -> Self { Self }
            fn runtime<T>(&self, error: $crate::error::CapError, context: &CapErrContext, retry: Option<&dyn Fn() -> Result<T, $crate::error::CapError>>) -> T {
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
            $handler.runtime($result(e), $handler.context(), None)
        } else { res.unwrap() }
    }};

    // For use on results with other errors
    (($handler:ident) $action:expr => $result:expr) => {{
        let res: Result<_, _> = $action;
        if let Err(e) = res {
            $handler.runtime($result(e), $handler.context(), Some(&|| {
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
            $handler.runtime(e, $handler.context(), None)
        } else { res.unwrap() }
    }};

    // For use on results with cap errors
    (($handler:ident) $action:expr) => {{
        let res: Result<_, $crate::error::CapError> = $action;
        if let Err(e) = res {
            $handler.runtime(e, $handler.context(), Some(&|| $action))
        } else { res.unwrap() }
    }};
}