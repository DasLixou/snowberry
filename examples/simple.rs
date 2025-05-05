use std::{
    any::{TypeId, type_name_of_val},
    marker::PhantomData,
};

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
        f: F,
        _e: &E,
    ) -> Context<'a, ((), (Callchain<E>, F))> {
        unsafe { core::mem::transmute::<Context<'_, ()>, Context<'_, ((), (Callchain<E>, F))>>(cx) }
    }
    let cx = ding(
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

fn label<E>(text: &str) -> impl Composable<E> {
    composable!(move |cx| {
        let (cx, button_press) = event!(cx);
        // let _cx = cx.compose(implicit_event(button_press)); // We don't provide to the environment yet, so this doesn't compile.
        println!("With label: {text}");
    })
}

fn implicit_event<E>(event: impl Event<E>) -> impl Composable<E> {
    composable!(move |cx| {})
}

fn test<E>(e: E, event: impl Event<E>) {
    let (_, callback) = e.get();
    (callback)();
    println!("{}", type_name_of_val(&x));
}
