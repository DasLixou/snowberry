use std::error::Error;

use snowberry_core::{App, Constructs, Context, InitElement, Runner};
use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

pub struct WinitRunner;

impl Runner for WinitRunner {
    fn run(&mut self, _app: App, root: impl InitElement) -> Result<(), Box<dyn Error>> {
        let event_loop = EventLoop::<()>::with_user_event().build()?;

        let constructs = Constructs::new();
        let mut state = WinitRunnerState {
            init: root,
            constructs,
        };

        event_loop.run_app(&mut state).map_err(Into::into)
    }
}

pub struct WinitRunnerState<I: InitElement> {
    init: I,
    constructs: Constructs,
}

impl<I: InitElement> ApplicationHandler<()> for WinitRunnerState<I> {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        if StartCause::Init != cause {
            return;
        };

        let mut cx = Context {
            constructs: &mut self.constructs,
        };

        self.init.exec(&mut cx);

        println!("Root was built!");
    }

    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, _event: ()) {}
}
