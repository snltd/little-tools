#[macro_export]
macro_rules! verbose {
    ($opts:expr, $($arg:tt)*) => {
        if $opts.verbose || $opts.noop {
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! if_op {
    ($opts:expr, $op:expr) => {{ if !$opts.noop { $op } else { Ok(()) } }};
}
