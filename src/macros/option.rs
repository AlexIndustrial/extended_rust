#[macro_export]
macro_rules! none_return {
    ($expr:expr, $rt:expr) => {
        match $expr {
            Some(r) => r,
            None => return $rt,
        }
    };
}

#[macro_export]
macro_rules! some_return {
    ($expr:expr, $rt:expr) => {
        match $expr {
            Some(_) => return $rt,
            None => (),
        }
    };
}
