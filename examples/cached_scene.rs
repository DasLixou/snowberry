use std::cell::Cell;

use snowberry::composition::Composition;

fn main() {
    Composition::open(|store: _| {
        let (store, needs_update) = store.store_val(Cell::new(false));

        let (store, update_parent_action) = store.store_val(|| {
            if needs_update.get() {
                return;
            }
            needs_update.set(true);
            println!("Heavy update logic (OUTER)");
        });

        let (store, _) = store.store_val(Composition::open(|store: _| {
            let (store, needs_update) = store.store_val(Cell::new(false));

            let (store, update_parent_action) = store.store_val(|| {
                if needs_update.get() {
                    return;
                }
                needs_update.set(true);
                update_parent_action();
                println!("Heavy update logic (A)");
            });

            let (store, _) = store.store_val(Composition::open(|store: _| {
                update_parent_action();
                update_parent_action();
                update_parent_action();
                store.end();
            }));

            store.end();
        }));

        let (store, _) = store.store_val(Composition::open(|store: _| {
            let (store, needs_update) = store.store_val(Cell::new(false));

            let (store, update_parent_action) = store.store_val(|| {
                if needs_update.get() {
                    return;
                }
                needs_update.set(true);
                update_parent_action();
                println!("Heavy update logic (B)");
            });

            let (store, _) = store.store_val(Composition::open(|store: _| {
                update_parent_action();
                update_parent_action();
                update_parent_action();
                store.end();
            }));

            store.end();
        }));

        store.end();
    });
}
