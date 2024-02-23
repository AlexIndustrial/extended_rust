
#[macro_export]
macro_rules! await_ft {
    ($future:expr) => {
        $future.await
    };
}