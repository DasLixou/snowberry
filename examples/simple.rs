use std::marker::PhantomData;

use snowberry::{composable, composable_::Composable, context::Context, event, event::Event};

fn main() {
    label("Hello").compose(Context { env: PhantomData });
}

fn label(text: &str) -> impl Composable<()> {
    composable!(move |cx| {
        let (cx, button_press) = event!(cx);
        let _cx = cx.compose(inner(button_press));
        println!("With label: {text}");
    })
}

fn inner(event: impl Event) -> impl Composable<()> {
    composable!(move |_cx| {})
}
