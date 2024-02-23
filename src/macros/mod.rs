#[cfg(feature = "macro_async_fut")]
pub mod async_fut;
#[cfg(feature = "macro_convert")]
pub mod convert;
#[cfg(feature = "macro_option_returns")]
pub mod option;
#[cfg(feature = "macro_prints")]
pub mod out;
#[cfg(feature = "macro_result_returns")]
pub mod result;

#[macro_export]
macro_rules! rt {
    ($expr:expr) => {
        return $expr
    };
}
