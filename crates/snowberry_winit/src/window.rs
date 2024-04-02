use snowberry_core::{context::Context, element::Element};
use winit::window::WindowBuilder;

pub fn window<'scope>(cx: Context<'scope>, title: &'static str, _scope: impl Element) {
    /*let window = WindowBuilder::new()
        .with_title(title)
        .build(cx.runner_data)
        .unwrap();
    cx.scope.store(window);*/
}
