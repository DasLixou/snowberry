use crate::context::Context;

pub trait Composable<Env> {
    fn compose(self, cx: Context<Env>);
}

#[macro_export]
macro_rules! composable {
    ($($tokens:tt)*) => {{
        fn make_composable<F, Env>(f: F) -> impl $crate::composable_::Composable<Env>
        where
            F: FnOnce($crate::context::Context<Env>)
        {
            struct MyComposable<F, Env>(F, ::core::marker::PhantomData<Env>);
            impl<F, Env> $crate::composable_::Composable<Env> for MyComposable<F, Env>
            where
                F: FnOnce($crate::context::Context<Env>)
            {
                fn compose(self, cx: $crate::context::Context<Env>) {
                    (self.0)(cx);
                }
            }
            MyComposable(f, ::core::marker::PhantomData)
        }
        make_composable($($tokens)*)
    }};
}
