use snowberry_core::{Context, Runner, Snowberry};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Default)]
pub struct Windows(Vec<String>);

pub struct WinitRunner;

impl Runner for WinitRunner {
    fn run(self, mut snowberry: Snowberry) {
        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(ControlFlow::Wait);

        // TODO: update this and make it work via events so that it also works when creating windows after the runner started
        if let Some(windows) = snowberry.global_resources.get_mut::<Windows>() {
            for window in windows.0.drain(..) {
                let window = WindowBuilder::new()
                    .with_title(window)
                    .build(&event_loop)
                    .unwrap();
                std::mem::forget(window); // TODO: update this once Loaders are implemented
            }
        }

        event_loop
            .run(move |event, elwt| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    elwt.exit();
                }
                _ => (),
            })
            .unwrap();
    }
}

pub fn window(cx: Context, title: &str, sub: impl FnOnce(Context)) {
    cx.global_resources
        .entry::<Windows>()
        .or_insert_with(Default::default)
        .0
        .push(title.to_owned());
    sub(cx);
}
