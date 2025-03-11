use std::cell::Cell;

use snowberry::{composable_::Composable, composition::Composition};
use winit::{application::ApplicationHandler, event_loop::ActiveEventLoop};

thread_local! {
    pub(crate) static LOOP: Cell<Option<&'static ActiveEventLoop>> = Cell::new(None);
}

// TODO: get rid of that as a macro and make stuff private again
#[macro_export]
macro_rules! run_winit {
    ($($tokens:tt)*) => {
        let event_loop = ::winit::event_loop::EventLoop::new().unwrap();

        let mut app = $crate::runner::App {
            composable: Some($($tokens)*),
            composition: None,
        };
        event_loop.run_app(&mut app).unwrap();
        // TODO: when we never create a window, this doesn't close itself yet.
    };
}

pub struct App<'life, C: Composable<'life>> {
    // TODO: Change this to FnMut where we can request a new composable for every resume or just make Composable take `&mut self` instead of `self`
    pub composable: Option<C>,
    pub composition: Option<Composition<'life, C>>,
}

impl<'life, C: Composable<'life, Down = ()>> ApplicationHandler for App<'life, C> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        unsafe {
            LOOP.set(Some(core::mem::transmute(event_loop)));
        }
        self.composition = Some(Composition::root(self.composable.take().unwrap()));
        LOOP.set(None);
    }

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.composition = None;
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}
