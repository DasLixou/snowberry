use std::cell::Cell;

use snowberry::{composable, composable::Composable, simple::run_simple};

fn main() {
    run_simple(counter());
}

fn counter() -> impl Composable {
    composable!(|store| {
        let (store, count) = store.store_val(Cell::new(0));
        println!("Currently {}", count.get());
        store.end();
    })
}
