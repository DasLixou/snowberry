use std::cell::RefCell;

use snowberry::core::{dynamic::Dynamic, Context, Snowberry};
use snowberry::window::{window, WinitRunner};

fn main() {
    Snowberry::new().run(WinitRunner, content);
}

struct MyEvent;

fn content(cx: Context) {
    window(cx, "My Snowberry UI", |mut cx| {
        let counter = cx.deposit(RefCell::new(0));

        let on_click = cx.event();
        let listener = cx.listener(|_e| {
            *counter.borrow_mut() += 1;
            println!("Counter is now {}", counter.borrow());
        });
        cx.subscribe(on_click, listener);
        cx.publish(on_click, MyEvent);
        cx.publish(on_click, MyEvent);

        label(cx, "Hello, world!");
    })
}

fn label(_cx: Context, text: &str) {
    let mut counter = Dynamic::new(0);
    *counter.get_mut() += 1;
    *counter.get_mut() += 1;

    println!("{text}");
}
