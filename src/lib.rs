#![warn(rust_2018_idioms)]
#![warn(future_incompatible)]

pub mod composable;
pub mod composition;
pub mod ext_stack;
pub mod generics_stack;
pub mod option_ext;
pub mod reaction_chain;
pub mod responsible_pin;
pub mod scope;
pub mod simple;
pub mod store;
pub mod uninit;

pub mod prelude {
    pub use crate::scope::Scope;
}
