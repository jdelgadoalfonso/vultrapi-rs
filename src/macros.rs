#![allow(unused_macros)]

#[cfg(feature = "debug")]
macro_rules! debug {
    ($fmt:expr) => (println!(concat!("**DEBUG** ", $fmt)));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("**DEBUG** ",$fmt), $($arg)*));
}

#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($fmt:expr) => ();
    ($fmt:expr, $($arg:tt)*) => ();
}
