use std::{fmt, error::Error};
pub use crate::handler;
pub(crate) use crate::handle;

#[derive(Debug)]
pub enum CapErrContext<'a> {
    WhileZipping(&'a str),
    WhileZippingFile(&'a str),
    WhileBuildingTarBall(&'a str),
    Undefined,
}

#[derive(Debug)]
pub enum CapError {
    IOError(std::io::Error),
    WalkDirError(walkdir::Error),
    FileNotFound(String),
}

impl fmt::Display for CapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CapError::*;
        match self {
            FileNotFound(p) => write!(f, "File '{p}' not found"),
            IOError(e) => write!(f, "IO Error: {:?}", e),
            WalkDirError(e) => write!(f, "WalkDir Error: {:?}", e),
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
    pub fn runtime<TT>(&self, error: CapError, retry: Option<&dyn Fn() -> Result<TT, CapError>>) -> TT {
        self.handler.runtime(error, &self.context, retry)
    }
}

#[macro_export]
macro_rules! handler {
    ($($pattern:pat => $result:expr),* $(,)?) => {{
        use $crate::error::CapErrHandler;
        pub struct MyCapErrHandler;
        impl Copy for MyCapErrHandler {}
        impl Clone for MyCapErrHandler {
            #[inline]
            fn clone(&self) -> Self { Self::init() }
        }
        impl $crate::error::CapErrHandler for MyCapErrHandler {
            #[inline]
            fn init() -> Self { Self }
            fn runtime<T>(&self, error: $crate::error::CapError, context: &CapErrContext, retry: Option<&dyn Fn() -> Result<T, $crate::error::CapError>>) -> T {
                match (error, context, retry) {
                    $($pattern => $result),*
                }
            }
        }

        MyCapErrHandler::init()
    }}
}

#[macro_export]
macro_rules! handle {
    // For use on results with other errors (once)
    (($handler:ident) ($action:expr) => $result:expr) => {{
        let res: Result<_, _> = $action;
        if let Err(e) = res {
            $handler.runtime($result(e), None)
        } else { res.unwrap() }
    }};

    // For use on results with other errors
    (($handler:ident) $action:expr => $result:expr) => {{
        let res: Result<_, _> = $action;
        if let Err(e) = res {
            $handler.runtime($result(e), Some(&|| {
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
            $handler.runtime(e, None)
        } else { res.unwrap() }
    }};

    // For use on results with cap errors
    (($handler:ident) $action:expr) => {{
        let res: Result<_, $crate::error::CapError> = $action;
        if let Err(e) = res {
            $handler.runtime(e, Some(&|| $action))
        } else { res.unwrap() }
    }};
}