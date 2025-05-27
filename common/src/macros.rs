#[macro_export]
macro_rules! verbose {
    ($opts:expr, $($arg:tt)*) => {
        if $opts.verbose || $opts.noop {
            println!($($arg)*);
        }
    };
}
