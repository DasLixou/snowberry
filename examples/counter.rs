use std::{cell::Cell, pin::pin};

use snowberry::{composable, composable::Composable, snowberry};

fn main() {
    snowberry!(counter());
}

fn counter<'l>() -> impl Composable<'l> {
    composable!(|store| {
        let (store, count) = store.store_val(Cell::new(0));
        println!("Currently {}", count.get());
        store.end();
    })
}
