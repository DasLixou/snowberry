use std::mem::MaybeUninit;

use snowberry::{
    reactive::{chain_end, reactive},
    scope::Scope,
    store::Stored,
};

fn main() {
    let stored = MaybeUninit::uninit();

    Scope::open(|_scope| {
        Stored::store(stored, |store| {
            let (store, counter_chain, counter) = reactive(store, 0);
            counter.update(|c| *c += 1);
            chain_end(counter_chain);

            store.end();
        });
    });
}
