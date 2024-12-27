use std::marker::PhantomData;

use crate::store::Store;

/// Typically implemented on ZST type or a FnOnce wrapper when creation holds variables
pub trait Composable<'life> {
    type Store: Sized + 'life;

    fn compose(self, store: Store<'life, Self::Store>);
}

pub fn make_composable<'life, F, S>(closure: F) -> impl Composable<'life>
where
    F: FnOnce(Store<'life, S>) + 'life,
    S: 'life,
{
    struct MyComposable<'life, F, S> {
        closure: F,
        phantom: PhantomData<&'life S>,
    }
    impl<'life, F, S> Composable<'life> for MyComposable<'life, F, S>
    where
        F: FnOnce(Store<'life, S>) + 'life,
        S: 'life,
    {
        type Store = S;

        fn compose(self, store: Store<'life, Self::Store>) {
            (self.closure)(store);
        }
    }
    MyComposable::<F, S> {
        closure,
        phantom: PhantomData,
    }
}

#[macro_export]
macro_rules! composable {
    ($($tokens:tt)*) => {{
        ::snowberry::composable::make_composable($($tokens)*)
    }};
}
