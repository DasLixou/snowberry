use snowberry_core::BuildContext;

pub fn window(cx: &BuildContext, title: &str, sub: impl FnOnce(&BuildContext)) {
    println!("Creating window '{title}'");
    sub(cx);
}
