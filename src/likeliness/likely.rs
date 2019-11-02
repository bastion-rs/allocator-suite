/// likely.
#[macro_export]
macro_rules! likely {
    ($expr: expr) => {
        unsafe { ::std::intrinsics::likely($expr) }
    };
}
