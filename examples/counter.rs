use std::mem::MaybeUninit;

use snowberry::{reactive::reactive, scope::Scope, store::Stored};

fn main() {
    let stored = MaybeUninit::uninit();

    Scope::open(|_scope| {
        Stored::store(stored, |store| {
            let (store, counter_chain, counter) = reactive(store, 0);

            let counter_chain = counter_chain.react(|| {
                println!("Counter changed!");
            });
            counter_chain.end();

            counter.update(|c| *c += 1);

            store.end();
        });
    });
}
