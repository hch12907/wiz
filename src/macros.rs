#[macro_export]
macro_rules! get {
    ($x:expr, $y:expr) => (match $x {
        Ok(x) => x,
        Err(why) => return Err($y.to_string() + why.description()),
    });
}

#[macro_export]
macro_rules! get_option {
    ($x:expr, $y:expr) => (match $x {
        Some(x) => x,
        None => return Err($y.to_string()),
    });
}
