use std::mem::MaybeUninit;

use snowberry::{scope::Scope, store::Stored};

struct Bomb<'s>(&'s str);
impl Drop for Bomb<'_> {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

fn main() {
    let stored = MaybeUninit::uninit();

    Scope::open(|_scope| {
        Stored::store(stored, |store| {
            let (store, counter) = store.store_val(0);
            *counter += 1;
            let (store, _a) = store.store_val(Bomb("INNER a - First Bomb"));
            let (store, _b) = store.store_val(Bomb("INNER b - Second Bomb"));
            store.end();
        });
    });
}
