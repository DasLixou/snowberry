#![warn(rust_2018_idioms)]
#![warn(future_incompatible)]

pub mod composable;
pub mod composition;
pub mod context;
pub mod ext_stack;
pub mod generics_stack;
pub mod scope;
pub mod simple;

pub mod prelude {
    pub use crate::scope::Scope;
}
