#[macro_export]
macro_rules! string {
    () => {
        String::new()
    };
    ($expr:expr) => {
        $expr.to_string()
    };
}

#[macro_export]
macro_rules! to_option {
    ($res:expr) => {
        match $res {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    };
}

#[macro_export]
macro_rules! to_result {
    ($opt:expr) => {
        match $opt {
            Some(s) => Ok(s),
            None => Err(()),
        }
    };
    ($opt:expr, $on_err:expr) => {
        match $opt {
            Some(s) => Ok(s),
            None => Err($on_err),
        }
    };
}
