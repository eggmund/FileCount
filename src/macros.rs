// Super useful macro for printing error message and exiting program
#[macro_export]
macro_rules! error_message {
    ($error_code:expr) => { process::exit($error_code); };
    ($error_code:expr, $($arg:tt)*) => {
        eprintln!("fcount: {}", format_args!($($arg)*));
        process::exit($error_code);
    };
}