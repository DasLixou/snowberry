#![warn(rust_2018_idioms)]
#![warn(future_incompatible)]

use std::marker::PhantomData;

pub mod composable_;
pub mod context;
pub mod environment;
pub mod event;

pub type InvariantLifetime<'brand> = PhantomData<fn(&'brand ()) -> &'brand ()>;
