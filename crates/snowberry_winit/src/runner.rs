use std::{collections::HashMap, error::Error, mem::transmute};

use snowberry_core::{
    app::App, context::Context, element::Element, resource::Resources, runner::Runner, scope::Scope,
};
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{EventLoopBuilder, EventLoopWindowTarget},
    window::WindowId,
};

pub(crate) struct EventLoopContext<'elwt> {
    pub(crate) window_target: &'elwt EventLoopWindowTarget<()>,
}

pub(crate) struct Windows {
    pub(crate) event_handler: HashMap<WindowId, Box<dyn Fn(WindowEvent)>>,
}

pub struct WinitRunner;

impl Runner for WinitRunner {
    fn run(&mut self, _app: App, root: Box<dyn Element>) -> Result<(), Box<dyn Error>> {
        let event_loop = EventLoopBuilder::<()>::with_user_event().build()?;

        let mut root_scope = Scope::new();
        let mut resources = Resources::new();

        resources.insert(Windows {
            event_handler: HashMap::new(),
        });

        event_loop.run(move |event, elwt| {
            //println!("{event:?}");
            match event {
                Event::NewEvents(StartCause::Init) => {
                    let elc = EventLoopContext {
                        window_target: elwt,
                    };
                    // TODO: this should be safe but moved into with_temp and proper lifetime stuff
                    // - it is safe as long as it can't be removed or otherwise owned with a longer lifetime.
                    // - just getting is fine because the borrow can't live longer than the return value of with_temp
                    let elc: EventLoopContext<'static> = unsafe { transmute(elc) };
                    resources.with_temp(elc, |resources| {
                        root.build(&mut Context {
                            resources,
                            scope: &mut root_scope,
                        });
                    });

                    println!("Root was built!");
                }
                Event::WindowEvent { window_id, event } => {
                    let windows = resources.get::<Windows>().unwrap();
                    if let Some(handler) = windows.event_handler.get(&window_id) {
                        handler(event);
                    } else {
                        eprintln!("Handler for Window {window_id:?} not defined!");
                    }
                }
                _ => {}
            }
        })?;

        Ok(())
    }
}
