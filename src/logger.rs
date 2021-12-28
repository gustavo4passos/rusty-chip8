#[allow(unused_macros)]
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => (println!($($arg)*));
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => { };

}