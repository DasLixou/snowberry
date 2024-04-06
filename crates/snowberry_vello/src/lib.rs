use snowberry_core::{context::Context, resource::Resource};
use vello::{util::RenderContext, Renderer};
use wgpu::SurfaceTarget;

// TODO: can we get smth like bevy has for internal macro use? :3
mod snowberry {
    pub use snowberry_core as core;
}

#[derive(Resource)]
pub struct VelloContext {
    pub render_cx: RenderContext,
    pub renderers: Vec<Option<Renderer>>,
}

pub fn init(cx: &mut Context<'_, '_>) {
    let render_cx = RenderContext::new().unwrap();
    let renderers: Vec<Option<Renderer>> = vec![];

    cx.resources.insert(VelloContext {
        render_cx,
        renderers,
    });
}

pub fn create_surface<'scope>(
    cx: &mut Context<'scope, '_>,
    surface_target: impl Into<SurfaceTarget<'scope>> + 'scope,
) {
    let Some(vc) = cx.resources.get_mut::<VelloContext>() else {
        eprintln!("VelloContext isn't initialized yet!");
        return;
    };

    let surface_future =
        vc.render_cx
            .create_surface(surface_target, 100, 100, wgpu::PresentMode::AutoVsync);
    let _surface = pollster::block_on(surface_future).expect("Error creating surface");
}
