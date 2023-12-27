use snowberry::core::{BuildContext, Snowberry};

fn main() {
    Snowberry::new().run(content);
}

fn content(cx: &BuildContext) {
    label(cx, "Hello, world!");
}

fn label(_cx: &BuildContext, text: &str) {
    println!("{text}");
}
