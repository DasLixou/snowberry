use std::marker::PhantomData;

use crate::context::Context;

/// Typically implemented on ZST type or a FnOnce wrapper when creation holds variables
pub trait Composable<'life>: 'life {
    type Store: Sized + 'life;

    fn compose<'cx>(self, cx: Context<'cx, (), Self::Store>) -> Context<'cx, Self::Store, ()>;
}

pub fn make_composable<'life, F, S>(closure: F) -> impl Composable<'life>
where
    for<'cx> F: FnOnce(Context<'cx, (), S>) -> Context<'cx, S, ()> + 'life,
    S: 'life,
{
    struct MyComposable<'life, F, S> {
        closure: F,
        phantom: PhantomData<&'life S>,
    }
    impl<'life, F, S> Composable<'life> for MyComposable<'life, F, S>
    where
        for<'cx> F: FnOnce(Context<'cx, (), S>) -> Context<'cx, S, ()> + 'life,
        S: 'life,
    {
        type Store = S;

        fn compose<'cx>(self, cx: Context<'cx, (), Self::Store>) -> Context<'cx, Self::Store, ()> {
            (self.closure)(cx)
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
