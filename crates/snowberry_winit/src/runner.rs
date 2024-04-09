use std::{collections::HashMap, error::Error, marker::PhantomData, mem::transmute};

use slotmap::SlotMap;
use snowberry_core::{
    app::App,
    context::Context,
    element::Element,
    event_station::EventStation,
    resource::{Resource, Resources},
    runner::Runner,
    scope::{Scope, ScopeKey, ScopeLife},
};
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget},
    window::WindowId,
};

#[derive(Resource)]
#[snowberry_path = "internal"]
pub(crate) struct EventLoopContext<'elwt> {
    pub(crate) window_target: &'elwt EventLoopWindowTarget<()>,
}

#[derive(Resource)]
#[snowberry_path = "internal"]
pub struct EventQueue {
    pub proxy: EventLoopProxy<()>,
}

#[derive(Resource)]
#[snowberry_path = "internal"]
pub(crate) struct Windows {
    pub(crate) event_handler: HashMap<WindowId, EventStation<WindowEvent>>,
}

pub struct WinitRunner;

impl Runner for WinitRunner {
    fn run<'root>(&mut self, _app: App, root: impl Element<'root>) -> Result<(), Box<dyn Error>> {
        let event_loop = EventLoopBuilder::<()>::with_user_event().build()?;
        let proxy = event_loop.create_proxy();

        let mut resources = Resources::new();
        let mut scopes: SlotMap<ScopeKey, Scope> = SlotMap::with_key();
        let root_scope = scopes.insert(Scope::new());

        resources.insert(Windows {
            event_handler: HashMap::new(),
        });
        resources.insert(EventQueue { proxy });

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
                            life: ScopeLife(PhantomData),
                        });
                    });

                    println!("Root was built!");
                }
                Event::WindowEvent { window_id, event } => {
                    let windows = resources.get::<Windows>().unwrap();
                    if let Some(station) = windows.event_handler.get(&window_id) {
                        let station = station.clone(); // we need to clone in order to pass &mut resources :< is there a better way? resource locking maybe? or RefCells?
                        for (scope, listener) in &station.listeners {
                            listener.run(
                                &event,
                                &mut Context {
                                    resources: &mut resources, // TODO: we should also move elc in here when it has a better "safer" api
                                    scopes: &mut scopes,
                                    scope: *scope,
                                    life: ScopeLife(PhantomData),
                                },
                            );
                        }
                    } else {
                        eprintln!("Handler for Window {window_id:?} not defined!");
                    }
                }
                Event::UserEvent(_) => {
                    println!("Got user event!");
                }
                _ => {}
            }
        })?;

        Ok(())
    }
}
