pub mod error;
pub mod extensions;
pub mod logger;
pub mod panic;
pub mod string;
pub mod winrt;
pub mod elevation;
pub mod ordered_map;

#[macro_export]
macro_rules! cloned {
    ([$($vars:ident),+] $e:expr) => {
        {
            $( let $vars = $vars.clone(); )+
            $e
        }
    };
}

#[macro_export]
macro_rules! log_assert {
    ($cond:expr) => {{ if !$cond { log::warn!("Assertion failed: {}", stringify!($cond)); } }};
}