pub mod error;
pub mod extensions;
pub mod logger;
pub mod panic;
pub mod string;
pub mod winrt;

#[macro_export]
macro_rules! cloned {
    ([$($vars:ident),+] $e:expr) => {
        {
            $( let $vars = $vars.clone(); )+
            $e
        }
    };
}
