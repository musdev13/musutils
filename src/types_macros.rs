#[macro_export]
macro_rules! log {
    ($is_json:expr, $($arg:tt)*) => {
        if !$is_json {
            println!($($arg)*);
        }
    };
}
