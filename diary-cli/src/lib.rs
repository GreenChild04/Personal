pub use console::*;

pub mod cli;
pub mod build_config;
pub mod env;
pub mod console;
pub mod version;
pub mod hv;
pub mod error;
pub mod io;
pub mod lock;

#[macro_export]
macro_rules! niceif {
    ($condition:expr, $true_expr:expr, $false_expr:expr $(,)?) => {
        {
            if $condition {
                $true_expr
            } else {
                $false_expr
            }
        }
    };
    ($condition:expr, $true_expr:expr, $false_expr:expr) => {
        {
            if $condition {
                ($true_expr)
            } else {
                ($false_expr)
            }
        }
    };
}

#[macro_export]
macro_rules! bind {
    ($input:expr => $code:expr) => {
        $code($input)
    };
    ($input:ident = $code:expr) => {
        let $input = $code;
        &input
    }
}

#[macro_export]
macro_rules! post_process {
    ($input:expr => $code:expr) => {{
        let mut obj = $input;
        $code(&mut obj);
        return obj;
    }}
}