use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub struct Dynamic<T: Debug> {
    val: T,
}

impl<T: Debug> Dynamic<T> {
    pub fn new(val: T) -> Self {
        Self { val }
    }

    pub fn get(&self) -> &T {
        &self.val
    }

    pub fn get_mut(&mut self) -> MutHandle<'_, T> {
        MutHandle { val: &mut self.val }
    }
}

pub struct MutHandle<'v, T: Debug> {
    val: &'v mut T,
}

impl<'v, T: Debug> Drop for MutHandle<'v, T> {
    fn drop(&mut self) {
        println!("Value changed and is now {:?}", self.val)
    }
}

impl<'v, T: Debug> Deref for MutHandle<'v, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.val
    }
}

impl<'v, T: Debug> DerefMut for MutHandle<'v, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.val
    }
}
