use std::cell::Cell;

use snowberry::{composable, composable::Composable};
use snowberry_winit::{run_winit, window::window};

fn main() {
    run_winit!(counter());
}

fn counter<'l>() -> impl Composable<'l> {
    window(
        String::from("snowberry counter"),
        composable!(|store| {
            let (store, count) = store.store_val(Cell::new(0));
            println!("Currently {}", count.get());
            store.end();
        }),
    )
}
