use snowberry::core::{BuildContext, Snowberry};
use snowberry::event::{Event, Publisher};

fn main() {
    Snowberry::new().run(content);
}

#[derive(Clone, Copy)]
struct MyEvent;
impl Event for MyEvent {}

fn content(cx: &BuildContext) {
    let mut on_click = Publisher::new();
    on_click += |_e| {
        println!("Pressed");
    };
    on_click.publish(MyEvent);

    label(cx, "Hello, world!");
}

fn label(_cx: &BuildContext, text: &str) {
    println!("{text}");
}
