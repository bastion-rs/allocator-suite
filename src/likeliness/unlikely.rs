/// unlikely.
#[macro_export]
macro_rules! unlikely {
    ($expr: expr) => {
        unsafe { ::std::intrinsics::unlikely($expr) }
    };
}
