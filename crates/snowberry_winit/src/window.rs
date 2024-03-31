use snowberry_core::{context::Context, element::Element};
use winit::window::WindowBuilder;

use crate::WinitRunner;

pub fn window<'scope>(
    cx: Context<'scope, '_, WinitRunner>,
    title: &'static str,
    _scope: impl Element<WinitRunner>,
) {
    let window = WindowBuilder::new()
        .with_title(title)
        .build(cx.runner_data)
        .unwrap();
    cx.scope.store(window);
}
