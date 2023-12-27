mod build_cx;
pub use build_cx::BuildContext;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub struct Snowberry {
    pub event_loop: EventLoop<()>,
}

impl Snowberry {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new().unwrap(),
        }
    }

    pub fn run<F>(mut self, root: F)
    where
        F: FnOnce(&BuildContext),
    {
        root(&BuildContext {
            snowberry: &mut self,
        });

        self.event_loop.set_control_flow(ControlFlow::Wait);

        self.event_loop
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
