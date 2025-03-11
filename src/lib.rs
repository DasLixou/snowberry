#![warn(rust_2018_idioms)]
#![warn(future_incompatible)]

pub mod composable;
pub mod composition;
pub mod context;
pub mod environment;
pub mod ext_stack;
pub mod lens_table;
pub mod recursive;
pub mod scope;
pub mod simple;

pub mod prelude {
    pub use crate::scope::Scope;
}
