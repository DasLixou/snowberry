use std::{cell::Cell, mem::MaybeUninit};

use snowberry::{composable::Composable, composition::Composition};

macro_rules! composable {
    ($($($store_name:ident: $store_ty:ty),+ =>)? { $($tokens:tt)* }) => {{
        use std::mem::MaybeUninit;
        struct Magic;
        struct MagicStore {
            $($($store_name: MaybeUninit<$store_ty>,)+)?
        }
        impl Composable<'_> for Magic {
            type Store = MagicStore;
            fn compose(self, store: &'_ mut Self::Store) {
                let MagicStore { $($($store_name)+)? } = store;
                $($tokens)*
            }
        }
        Magic
    }};
}

fn main() {
    let composition = MaybeUninit::uninit();
    Composition::open(composition, counter());
}

fn counter<'l>() -> impl Composable<'l> {
    composable! {
        count: Cell<i32>
        => {
            let count = count.write(Cell::new(0));
            println!("Currently {}", count.get());
        }
    }
}
