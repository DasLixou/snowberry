use std::cell::Cell;

use snowberry::{composable::Composable, composition::Composition};

fn main() {
    Composition::open({
        struct Counter;
        impl Composable<'_> for Counter {
            type Store = ((), Cell<i32>);
            fn compose(self, store: snowberry::store::Store<'_, Self::Store>) {
                let (store, count) = store.store_val(Cell::new(0));
            }
        }
        Counter
    });
}
