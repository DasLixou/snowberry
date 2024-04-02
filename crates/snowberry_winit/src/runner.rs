use std::{error::Error, mem::transmute};

use snowberry_core::{
    app::App, context::Context, element::Element, resource::Resources, runner::Runner, scope::Scope,
};
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{EventLoopBuilder, EventLoopWindowTarget},
};

pub(crate) struct EventLoopContext<'elwt> {
    pub(crate) window_target: &'elwt EventLoopWindowTarget<()>,
}

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
                    let elc = EventLoopContext {
                        window_target: elwt,
                    };
                    // TODO: is this safe? O.o
                    let elc: EventLoopContext<'static> = unsafe { transmute(elc) };
                    resources.with_temp(elc, |resources| {
                        root.build(Context {
                            resources,
                            scope: &mut root_scope,
                        });
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
