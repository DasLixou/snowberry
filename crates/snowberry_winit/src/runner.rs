use std::{cell::RefCell, collections::HashMap, error::Error, marker::PhantomData, mem::transmute};

use slotmap::SlotMap;
use snowberry_core::{
    app::App,
    context::Context,
    element::Element,
    event_station::{ErasedEventStation, EventDispatcher, EventStation, SubscriptionEntry},
    resource::{Resource, Resources},
    runner::Runner,
    scope::{Scope, ScopeKey, ScopeLife},
};
use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::WindowId,
};

#[derive(Resource)]
#[snowberry_path = "internal"]
pub(crate) struct EventLoopContext<'elwt> {
    pub(crate) active: &'elwt ActiveEventLoop,
}

#[derive(Resource)]
#[snowberry_path = "internal"]
pub(crate) struct Windows {
    pub(crate) event_handler: HashMap<WindowId, *const RefCell<EventStation<WindowEvent>>>,
}

pub struct WinitEventDispatcher(EventLoopProxy<ErasedEventStation>);
impl EventDispatcher for WinitEventDispatcher {
    fn dispatch(&self, erased_station: ErasedEventStation) {
        self.0
            .send_event(erased_station)
            .unwrap_or_else(|_| panic!("failed to send event :<"));
    }
}

pub struct WinitRunner;

impl Runner for WinitRunner {
    fn run<'root>(&mut self, _app: App, root: impl Element<'root>) -> Result<(), Box<dyn Error>> {
        let event_loop = EventLoop::<ErasedEventStation>::with_user_event().build()?;
        let proxy = event_loop.create_proxy();

        let mut resources = Resources::new();
        let mut scopes: SlotMap<ScopeKey, Scope> = SlotMap::with_key();
        let root_scope = scopes.insert(Scope::new());

        resources.insert(Windows {
            event_handler: HashMap::new(),
        });

        let event_dispatcher = WinitEventDispatcher(proxy);

        let mut state = WinitRunnerState {
            resources,
            scopes,
            root_scope,
            root,
            event_dispatcher,
            phantom: PhantomData,
        };

        event_loop.run_app(&mut state).map_err(Into::into)
    }
}

pub struct WinitRunnerState<'root, R: Element<'root>> {
    resources: Resources,
    scopes: SlotMap<ScopeKey, Scope>,
    root_scope: ScopeKey,
    root: R,
    event_dispatcher: WinitEventDispatcher,
    phantom: PhantomData<&'root ()>,
}

impl<'root, R: Element<'root>> ApplicationHandler<ErasedEventStation>
    for WinitRunnerState<'root, R>
{
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        if StartCause::Init != cause {
            return;
        };

        let elc = EventLoopContext { active: event_loop };
        // TODO: this should be safe but moved into with_temp and proper lifetime stuff
        // - it is safe as long as it can't be removed or otherwise owned with a longer lifetime.
        // - just getting is fine because the borrow can't live longer than the return value of with_temp
        let elc: EventLoopContext<'static> = unsafe { transmute(elc) };
        self.resources.with_temp(elc, |resources| {
            self.root.build(&mut Context {
                resources,
                scopes: &mut self.scopes,
                scope: self.root_scope,
                life: ScopeLife(PhantomData),
                event_dispatcher: &mut self.event_dispatcher,
            });
        });

        println!("Root was built!");
    }

    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let windows = self.resources.get::<Windows>().unwrap();
        if let Some(station) = windows.event_handler.get(&window_id) {
            let station = (*(unsafe { station.as_ref() }.unwrap()).borrow()).clone(); // we need to clone in order to pass &mut resources :< is there a better way? resource locking maybe? or RefCells?
            for SubscriptionEntry { scope, listener } in station.listeners.values() {
                // TODO: is that needed here?
                if !self.scopes.contains_key(*scope) {
                    continue;
                }
                listener.run(
                    event.clone(),
                    &mut Context {
                        resources: &mut self.resources, // TODO: we should also move elc in here when it has a better "safer" api
                        scopes: &mut self.scopes,
                        scope: *scope,
                        life: ScopeLife(PhantomData),
                        event_dispatcher: &mut self.event_dispatcher,
                    },
                );
            }
        } else {
            eprintln!("Handler for Window {window_id:?} not defined!");
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: ErasedEventStation) {
        let station = event;
        for (scope, listener) in station.listener_calls {
            if !self.scopes.contains_key(scope) {
                continue;
            }
            listener.run(&mut Context {
                resources: &mut self.resources, // TODO: we should also move elc in here when it has a better "safer" api
                scopes: &mut self.scopes,
                scope,
                life: ScopeLife(PhantomData),
                event_dispatcher: &mut self.event_dispatcher,
            });
        }
    }
}
