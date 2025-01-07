use std::{cell::Cell, mem::MaybeUninit, pin::Pin};

use snowberry::{composable, composable::Composable};
use snowberry_winit::{run_winit, window::window};

fn main() {
    let a = Bomb("Heloo");
    run_winit!(counter());
    let _ = a;
}

fn counter<'l>() -> impl Composable<'l> {
    window(
        String::from("snowberry counter"),
        composable!(|store| {
            let (store, count) = store.store_val(Cell::new(0));

            let (store, _) = store.store_val(Bomb("root!"));

            //panic!();

            //let (store, _): (_, Pin<&mut MaybeUninit<Bomb>>) = store.split_off();

            let store = store.compose(button("-"));
            let store = store.compose(button("+"));

            println!("Currently {}", count.get());
            store.end();
        }),
    )
}

fn button<'l>(label: &'l str) -> impl Composable<'l> {
    composable!(move |store| {
        println!("Button '{label}'");
        let (store, _) = store.store_val(Bomb("a button :3"));
        store.end();
    })
}

struct Bomb(&'static str);

impl Drop for Bomb {
    fn drop(&mut self) {
        println!("kapow '{}'", self.0);
    }
}
