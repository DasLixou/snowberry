use snowberry_core::{context::Context, element::Element};
use winit::window::WindowBuilder;

use crate::EventLoopContext;

pub fn window<'scope>(cx: Context<'scope, '_>, title: &'static str, _scope: impl Element) {
    let Some(elc) = cx.resources.get_mut::<EventLoopContext>() else {
        eprintln!("Can't get EventLoopContext!");
        return;
    };
    let window = WindowBuilder::new()
        .with_title(title)
        .build(elc.window_target)
        .unwrap();
    cx.scope.store(window);
}
