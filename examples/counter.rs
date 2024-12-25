use std::{cell::Cell, mem::MaybeUninit};

use snowberry::{composable, composable::Composable, composition::Composition};

fn main() {
    let mut composition = MaybeUninit::uninit();
    Composition::open(&mut composition, counter());
}

fn counter<'l>() -> impl Composable<'l> {
    composable!(|store| {
        let (store, count) = store.store_val(Cell::new(0));
        println!("Currently {}", count.get());
        store.end();
    })
}
