use std::{collections::HashMap, error::Error, mem::transmute};

use slotmap::SlotMap;
use snowberry_core::{
    app::App,
    context::Context,
    element::Element,
    event_station::EventStation,
    resource::{Resource, Resources},
    runner::Runner,
    scope::{Scope, ScopeKey},
};
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{EventLoopBuilder, EventLoopWindowTarget},
    window::WindowId,
};

// TODO: can we get smth like bevy has for internal macro use? :3
mod snowberry {
    pub use snowberry_core as core;
}

#[derive(Resource)]
pub(crate) struct EventLoopContext<'elwt> {
    pub(crate) window_target: &'elwt EventLoopWindowTarget<()>,
}

#[derive(Resource)]
pub(crate) struct Windows {
    pub(crate) event_handler: HashMap<WindowId, EventStation<WindowEvent>>,
}

pub struct WinitRunner;

impl Runner for WinitRunner {
    fn run(&mut self, _app: App, root: Box<dyn Element>) -> Result<(), Box<dyn Error>> {
        let event_loop = EventLoopBuilder::<()>::with_user_event().build()?;

        let mut resources = Resources::new();
        let mut scopes: SlotMap<ScopeKey, Scope> = SlotMap::with_key();
        let root_scope = scopes.insert(Scope::new());

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
                            scopes: &mut scopes,
                            scope: root_scope,
                        });
                    });

                    println!("Root was built!");
                }
                Event::WindowEvent { window_id, event } => {
                    let windows = resources.get::<Windows>().unwrap();
                    if let Some(station) = windows.event_handler.get(&window_id) {
                        station.run(event);
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
