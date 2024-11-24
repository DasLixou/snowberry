use snowberry::{composition::Composition, reactive::reactive};

fn main() {
    Composition::open(|store| {
        let (store, counter_chain, counter) = reactive(store, 0);

        let counter_chain = counter_chain.react(|| {
            println!("Counter changed!");
        });

        counter_chain.end();
        counter.update(|c| *c += 1);

        store.end();
    });
}
