use std::cell::Cell;

use snowberry::{
    composition::Composition,
    reaction_chain::{Reaction, ReactionChain},
};

fn main() {
    Composition::open(|store: _| {
        let (store, counter) = store.store_val(Cell::new(0));
        let (store, counter_chain, increment) = ReactionChain::on(store);

        let counter_chain = counter_chain.react(|| {
            let new = counter.get() + 1;
            counter.set(new);
            println!("Counter is now {new}!");
        });
        counter_chain.end();

        assert_eq!(counter.get(), 0);
        increment.invoke();
        assert_eq!(counter.get(), 1);
        increment.invoke();
        assert_eq!(counter.get(), 2);

        store.end();
    });
}
