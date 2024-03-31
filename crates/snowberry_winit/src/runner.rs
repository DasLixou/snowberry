use std::error::Error;

use snowberry_core::{app::App, context::Context, element::Element, runner::Runner, scope::Scope};
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{EventLoopBuilder, EventLoopWindowTarget},
};

pub struct WinitRunner;

impl Runner for WinitRunner {
    type Data<'data> = &'data EventLoopWindowTarget<()>;

    fn run(&mut self, _app: App, root: Box<dyn Element<Self>>) -> Result<(), Box<dyn Error>> {
        let event_loop = EventLoopBuilder::<()>::with_user_event().build()?;

        let mut root_scope = Scope::new();

        event_loop.run(move |event, elwt| {
            //println!("{event:?}");
            match event {
                Event::NewEvents(StartCause::Init) => {
                    root.build(Context {
                        runner_data: elwt,
                        scope: &mut root_scope,
                    });
                    println!("Root was built!");
                }
                Event::WindowEvent { event, .. } => match event {
                    // TODO: send that to the window
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    _ => {}
                },
                _ => {}
            }
        })?;

        Ok(())
    }
}
