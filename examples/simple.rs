use std::marker::PhantomData;

use snowberry::{composable, composable_::Composable, context::Context};

fn main() {
    label("Hello").compose(Context { env: PhantomData });
}

fn label(text: &str) -> impl Composable<()> {
    composable!(move |_cx| {
        println!("With label: {text}");
    })
}
