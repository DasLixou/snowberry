use std::marker::PhantomData;

use crate::context::Context;

/// Typically implemented on ZST type or a FnOnce wrapper when creation holds variables
pub trait Composable<'life>: 'life {
    type Store: Sized + 'life;

    fn compose<'cx, Down: 'cx + Copy, Env: 'cx + Copy>(
        self,
        cx: Context<'life, 'cx, (), Down, Env>,
    ) -> Context<'life, 'cx, Self::Store, Down, Env>
    where
        'life: 'cx;
}

// pub fn make_composable<'life, F, S, D>(closure: F) -> impl Composable<'life, Down = D>
// where
//     for<'cx> F: FnOnce(Context<'cx, (), D>) -> Context<'cx, S, D> + 'life,
//     S: 'life,
//     D: Copy + 'life,
//     E: 'life,
// {
//     struct MyComposable<'life, F, S, D, E> {
//         closure: F,
//         phantom: PhantomData<&'life (S, D, E)>,
//     }
//     impl<'life, F, S, D: Copy, E> Composable<'life> for MyComposable<'life, F, S, D, E>
//     where
//         for<'cx> F: FnOnce(Context<'cx, (), D, E>) -> Context<'cx, S, D, E> + 'life,
//         S: 'life,
//     {
//         type Store = S;
//         type Down = D;
//         type Env = E;

//         fn compose<'cx>(self, cx: Context<'cx, (), D, E>) -> Context<'cx, Self::Store, D, E> {
//             (self.closure)(cx)
//         }
//     }
//     MyComposable::<F, S, D, E> {
//         closure,
//         phantom: PhantomData,
//     }
// }

#[macro_export]
macro_rules! composable {
    ($($tokens:tt)*) => {{
        fn closure_comp<'s, 'cx, Store, Down: 'cx + Copy, Env: Copy>(
            cx: $crate::context::Context<'s, 'cx, (), Down, Env>,
            closure: impl FnOnce($crate::context::Context<'s, 'cx, (), Down, Env>) -> $crate::context::Context<'s, 'cx, Store, Down, Env>
        ) -> $crate::context::Context<'s, 'cx, Store, Down, Env> {
            closure(cx)
        }
        fn make_composable<'l, Fn, S: 'l>(_closure: Fn) -> impl $crate::composable_::Composable<'l>
        where
            for<'cx> Fn: FnOnce($crate::context::Context<'l, 'cx, (), (), $crate::environment::EnvTest>) -> $crate::context::Context<'l, 'cx, S, (), $crate::environment::EnvTest>
        {
            struct MyComposable<'life, S: 'life> {
                phantom: ::core::marker::PhantomData<&'life S>,
            }
            impl<'life, S: 'life> $crate::composable_::Composable<'life> for MyComposable<'life, S>
            {
                type Store = S;

                fn compose<'cx, Down: 'cx + Copy, Env: 'cx + Copy>(self, cx: $crate::context::Context<'life, 'cx, (), Down, Env>) -> $crate::context::Context<'life, 'cx, Self::Store, Down, Env>
                where
                    'life: 'cx {
                    let comp = closure_comp::<'life, 'cx, _, Down, Env>(cx, $($tokens)*);
                    unsafe {
                        $crate::composable_::transmute_almost_unchecked::<
                            $crate::context::Context<'life, 'cx, _, Down, Env>,
                            $crate::context::Context<'life, 'cx, Self::Store, Down, Env>,
                        >(comp)
                    }
                }
            }
            MyComposable::<'l, S> {
                phantom: ::core::marker::PhantomData,
            }
        }
        make_composable($($tokens)*)
        // struct ClosureTester<Fn, S>(::core::marker::PhantomData<(Fn, S)>) where for<'cx> Fn: FnOnce($crate::context::Context<'cx, (), (), $crate::environment::EnvTest>) -> $crate::context::Context<'cx, S, (), $crate::environment::EnvTest>;
        // impl<Fn, S> ClosureTester<Fn, S>
        //     where for<'cx> Fn: FnOnce($crate::context::Context<'cx, (), (), $crate::environment::EnvTest>) -> $crate::context::Context<'cx, S, (), $crate::environment::EnvTest>
        // {
        //     fn new(f: Fn) -> Self {
        //         Self(::core::marker::PhantomData)
        //     }
        // }
        // trait BecauseInherentAreUnstable {
        //     type Store;
        // }
        // impl<Fn, S> BecauseInherentAreUnstable for ClosureTester<Fn, S>
        //     where for<'cx> Fn: FnOnce($crate::context::Context<'cx, (), (), $crate::environment::EnvTest>) -> $crate::context::Context<'cx, S, (), $crate::environment::EnvTest>
        // {
        //     type Store = S;
        // }
        // struct MyComposable<'life, Because: BecauseInherentAreUnstable> {
        //     phantom: ::core::marker::PhantomData<&'life ()>,
        //     because: Because,
        // }
        // impl<'life, Because: BecauseInherentAreUnstable + 'life> $crate::composable_::Composable<'life> for MyComposable<'life, Because>
        // {
        //     type Store = Because::Store;

        //     fn compose<'cx, Down: Copy, Env: Copy>(self, cx: $crate::context::Context<'cx, (), Down, Env>) -> $crate::context::Context<'cx, Self::Store, Down, Env> {
        //         fn closure_comp<'cx, Store, Down: Copy, Env>(
        //             cx: $crate::context::Context<'cx, (), Down, Env>,
        //             closure: impl FnOnce($crate::context::Context<'cx, (), Down, Env>) -> $crate::context::Context<'cx, Store, Down, Env>
        //         ) -> $crate::context::Context<'cx, Store, Down, Env> {
        //             closure(cx)
        //         }
        //         closure_comp::<'cx, Self::Store, Down, Env>(cx, $($tokens)*)
        //     }
        // }
        // MyComposable {
        //     phantom: ::core::marker::PhantomData,
        //     because: ClosureTester::from($($tokens)*)
        // }
    }};
}

pub unsafe fn transmute_almost_unchecked<T, U>(value: T) -> U {
    use core::mem::{ManuallyDrop, size_of, transmute_copy};
    const { assert!(size_of::<T>() == size_of::<U>()) };
    unsafe { transmute_copy::<T, U>(&*ManuallyDrop::new(value)) }
}
