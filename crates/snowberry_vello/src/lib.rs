use snowberry_core::{context::Context, resource::Resource};
use vello::{util::RenderContext, Renderer};

// TODO: can we get smth like bevy has for internal macro use? :3
mod snowberry {
    pub use snowberry_core as core;
}

#[derive(Resource)]
pub struct VelloContext {
    pub render_cx: RenderContext,
    pub renderers: Vec<Option<Renderer>>,
}

pub fn init_vello(cx: &mut Context<'_, '_>) {
    let render_cx = RenderContext::new().unwrap();
    let renderers: Vec<Option<Renderer>> = vec![];

    cx.resources.insert(VelloContext {
        render_cx,
        renderers,
    });
}
