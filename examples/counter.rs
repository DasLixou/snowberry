use std::{cell::Cell, mem::MaybeUninit};

use snowberry::{
    composable::{make_composable, Composable},
    composition::Composition,
};

fn main() {
    let composition = MaybeUninit::uninit();
    Composition::open(composition, counter());
}

fn counter<'l>() -> impl Composable<'l> {
    make_composable(|store| {
        let count = store.write(Cell::new(0));
        println!("Currently {}", count.get());
        store.close();
    })
}
