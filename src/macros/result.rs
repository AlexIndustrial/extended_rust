#[macro_export]
macro_rules! err_return {
    ($expr:expr, $rt:expr) => {
        match $expr {
            Ok(r) => r,
            Err(_) => return $rt,
        }
    };
}

#[macro_export]
macro_rules! ok_return {
    ($expr:expr, $rt:expr) => {
        match $expr {
            Ok(_) => return $rt,
            Err(e) => e,
        }
    };
}

#[macro_export]
macro_rules! ok {
    ($expr:expr) => {
        return Ok($expr)
    };
}

#[macro_export]
macro_rules! err {
    ($expr:expr) => {
        return Err($expr)
    };
}

#[macro_export]
macro_rules! io_result {
    ($res_type:tt) => {
        std::io::Result<$res_type>
    };
}
