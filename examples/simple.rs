use std::marker::PhantomData;

use snowberry::{
    composable,
    composable_::Composable,
    context::Context,
    event,
    event_::{Callchain, Event},
};

fn main() {
    let cx = Context::<()> { env: &() };
    // label("Hello").compose(cx);
    let (cx, e) = event!(cx);
    fn ding<'a, F: Fn(), E>(
        cx: Context<'a, ()>,
        _f: F,
        _e: &E,
    ) -> Context<'a, ((), (Callchain<E>, F))> {
        unsafe { core::mem::transmute::<Context<'_, ()>, Context<'_, ((), (Callchain<E>, F))>>(cx) }
    }
    let _cx = ding(
        cx,
        || {
            println!("EVENT!! HEHE :D");
        },
        &e,
    );
    test(
        (
            (),
            (
                Callchain {
                    phantom: PhantomData,
                },
                || {
                    println!("EVENT!! HEHE :D");
                },
            ),
        ),
        e,
    );
}

fn test<E>(e: E, event: impl Event<E>) {
    let callback = e.get();
    (callback)();
}

// fn label<E>(text: &str) -> impl Composable<E> {
//     composable!(move |cx| {
//         let (cx, button_press) = event!(cx);
//         // let _cx = cx.compose(implicit_event(button_press)); // We don't provide to the environment yet, so this doesn't compile.
//         println!("With label: {text}");
//     })
// }

// fn implicit_event<E>(event: impl Event<E>) -> impl Composable<E> {
//     composable!(move |cx| {})
// }
