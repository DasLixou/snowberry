use snowberry::core::{BuildContext, Snowberry};
use snowberry::event::{Event, Publisher};
use snowberry::window::{window, WinitRunner};

fn main() {
    Snowberry::new().run(WinitRunner, content);
}

#[derive(Clone, Copy)]
struct MyEvent;
impl Event for MyEvent {}

fn content(cx: BuildContext) {
    window(cx, "My Snowberry UI", |cx| {
        let mut on_click = Publisher::new();
        on_click += |_e| {
            println!("Pressed");
        };
        on_click.publish(MyEvent);

        label(cx, "Hello, world!");
    })
}

fn label(_cx: BuildContext, text: &str) {
    println!("{text}");
}
