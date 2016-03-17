#![crate_type = "lib"]
#![crate_name = "tlpirust"]

// TODO: maybe output to stderr instead
#[macro_export]
macro_rules! logexit(
    ($code:expr, $($args:tt)*) => (
        {
            println!($($args)*);
            std::process::exit($code);
        }
    )
);
