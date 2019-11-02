#[macro_use]
pub mod likely;
#[macro_use]
pub mod unlikely;

#[macro_use]
pub mod prelude {
    pub use super::likely::*;
    pub use super::unlikely::*;
}
