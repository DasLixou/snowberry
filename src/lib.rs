#![warn(rust_2018_idioms)]
#![warn(future_incompatible)]

pub mod composable;
pub mod composition;
pub mod reactive;
pub mod scope;

pub mod prelude {
    pub use crate::scope::Scope;
}
