use std::cell::RefCell;

use snowberry::core::{Context, Snowberry};
use snowberry::event::{Event, Publisher};
use snowberry::window::{window, WinitRunner};

fn main() {
    Snowberry::new().run(WinitRunner, content);
}

#[derive(Clone, Copy)]
struct MyEvent;
impl Event for MyEvent {}

fn content(cx: Context) {
    window(cx, "My Snowberry UI", |cx| {
        let counter = cx.deposit(RefCell::new(0));

        let mut on_click = Publisher::new();
        on_click += |_e| {
            *counter.borrow_mut() += 1;
            println!("Counter is now {}", counter.borrow());
        };
        on_click.publish(MyEvent);
        on_click.publish(MyEvent);

        label(cx, "Hello, world!");
    })
}

fn label(_cx: Context, text: &str) {
    println!("{text}");
}
