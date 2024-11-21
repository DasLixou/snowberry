use std::mem::MaybeUninit;

use snowberry::{
    composable::{end, store_val, Composable},
    scope::Scope,
};

struct Bomb<'s>(&'s str);
impl Drop for Bomb<'_> {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

fn main() {
    let mut store = MaybeUninit::uninit();

    let _a = Bomb("OUTER a - First Bomb");
    let _b = Bomb("OUTER b - Second Bomb");

    Scope::open(|_scope| {
        (|store| {
            let (store, counter) = store_val(store, 0);
            *counter += 1;
            let (store, _a) = store_val(store, Bomb("INNER a - First Bomb"));
            let (store, _b) = store_val(store, Bomb("INNER b - Second Bomb"));
            end(store);
        })
        .compose(&mut store);
    });
    unsafe {
        store.assume_init();
    }
}
