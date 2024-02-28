pub mod error;
pub mod logger;
pub mod panic;
pub mod extensions;
pub mod string;

#[macro_export]
macro_rules! cloned {
    ([$($vars:ident),+] $e:expr) => {
        {
            $( let $vars = $vars.clone(); )+
            $e
        }
    };
}
