use std::mem::MaybeUninit;

use snowberry::{
    composable::{end, store_val, Composable},
    scope::Scope,
};

fn main() {
    let mut store = MaybeUninit::uninit();
    Scope::open(|_scope| {
        (|store| {
            let (counter, store) = store_val(0, store);
            *counter += 1;
            end(store);
        })
        .compose(&mut store);
    });
}
