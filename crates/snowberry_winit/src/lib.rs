use snowberry_core::BuildContext;
use winit::window::WindowBuilder;

pub fn window(cx: &BuildContext, title: &str, sub: impl FnOnce(&BuildContext)) {
    println!("Creating window '{title}'");
    let window = WindowBuilder::new()
        .with_title(title)
        .build(&cx.snowberry.event_loop)
        .unwrap();
    std::mem::forget(window); // TODO: update this once Loaders are implemented
    sub(cx);
}
