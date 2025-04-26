use std::marker::PhantomData;

use snowberry::{composable, composable_::Composable, context::Context, event, event_::Event};

fn main() {
    label("Hello").compose(Context::<()> { env: PhantomData });
}

fn label<E>(text: &str) -> impl Composable<E> {
    composable!(move |cx| {
        let (cx, button_press) = event!(cx);
        // let _cx = cx.compose(implicit_event(button_press)); // We don't provide to the environment yet, so this doesn't compile.
        println!("With label: {text}");
    })
}

fn implicit_event<E>(event: impl Event<E>) -> impl Composable<E> {
    composable!(move |_cx| {})
}
