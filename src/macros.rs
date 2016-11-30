#[macro_export]
macro_rules! get {
    ($x:expr, $y:expr) => (match $x {
        Ok(x) => x,
        Err(why) => return Err($y.to_string() + why.description()),
    });
}
