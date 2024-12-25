use std::marker::PhantomData;

use crate::store::Store;

/// Typically implemented on ZST type or a FnOnce wrapper when creation holds variables
pub trait Composable {
    type Store<'life>: Sized + 'life;

    fn compose<'life>(self, store: Store<'life, Self::Store<'life>>);
}

pub fn make_composable<F, S>(closure: F) -> impl Composable
where
    for<'life> F: FnOnce(Store<'life, S>) + 'life,
    for<'life> S: 'life,
{
    struct MyComposable<F, S> {
        closure: F,
        phantom: PhantomData<S>,
    }
    impl<F, S> Composable for MyComposable<F, S>
    where
        for<'life> F: FnOnce(Store<'life, S>) + 'life,
        for<'life> S: 'life,
    {
        type Store<'life> = S;

        fn compose<'life>(self, store: Store<'life, Self::Store<'life>>) {
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
