pub use console::*;

pub mod cli;
pub mod build_config;
pub mod env;
pub mod console;
pub mod version;
pub mod hv;
pub mod error;
pub mod io;

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