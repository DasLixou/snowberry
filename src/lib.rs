#![warn(rust_2018_idioms)]
#![warn(future_incompatible)]

pub mod composable;
pub mod composition;
pub mod scope;
pub mod store;

pub mod prelude {
    pub use crate::scope::Scope;
}
