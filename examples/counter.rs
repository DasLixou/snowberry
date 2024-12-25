use std::{cell::Cell, mem::MaybeUninit, pin::pin};

use snowberry::{composable, composable::Composable, composition::Composition};

fn main() {
    let composition = pin!(MaybeUninit::uninit());
    Composition::open(composition, counter());
}

fn counter<'l>() -> impl Composable<'l> {
    composable!(|store| {
        let (store, count) = store.store_val(Cell::new(0));
        println!("Currently {}", count.get());
        store.end();
    })
}
