
#[macro_export]
macro_rules! q_print_obj {
    ($obj:expr) => {
        println!("{:#?}", $obj)
    };
}

#[macro_export]
macro_rules! q_print {
    ($obj:expr) => {
        println!("{}", $obj)
    };
}