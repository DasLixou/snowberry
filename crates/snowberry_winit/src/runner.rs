use std::error::Error;

use snowberry_core::{
    app::App, context::Context, element::Element, resource::Resources, runner::Runner, scope::Scope,
};
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::EventLoopBuilder,
};

pub struct WinitRunner;

impl Runner for WinitRunner {
    fn run(&mut self, _app: App, root: Box<dyn Element>) -> Result<(), Box<dyn Error>> {
        let event_loop = EventLoopBuilder::<()>::with_user_event().build()?;

        let mut root_scope = Scope::new();
        let mut resources = Resources::new();

        event_loop.run(move |event, elwt| {
            //println!("{event:?}");
            match event {
                Event::NewEvents(StartCause::Init) => {
                    root.build(Context {
                        resources: &mut resources,
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
