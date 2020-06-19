/// likely.
#[macro_export]
macro_rules! likely {
    ($expr: expr) => {
        ::std::intrinsics::likely($expr)
    };
}
