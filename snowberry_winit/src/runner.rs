use std::cell::Cell;

use snowberry::{
    composable::Composable, composition::Composition, responsible_pin::ResponsiblePin,
};
use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop},
};

thread_local! {
    pub(crate) static LOOP: Cell<Option<&'static ActiveEventLoop>> = Cell::new(None);
}

// TODO: get rid of that as a macro and make stuff private again
#[macro_export]
macro_rules! run_winit {
    ($($tokens:tt)*) => {
        let event_loop = ::winit::event_loop::EventLoop::new().unwrap();

        ::snowberry::responsible_pin!(let unsafe composition = None);
        //let composition = ::std::pin::pin!(::std::mem::MaybeUninit::uninit());

        let mut app = $crate::runner::App {
            composable: Some($($tokens)*),
            composition,
        };
        event_loop.run_app(&mut app).unwrap();
        // TODO: when we never create a window, this doesn't close itself yet.
    };
}
/*pub fn run_winit(c: impl for<'life> Composable<'life>) {
    let event_loop = EventLoop::new().unwrap();

    let composition = pin!(MaybeUninit::uninit());

    let mut app = App {
        composable: Some(c),
        composition: Some(composition),
    };
    event_loop.run_app(&mut app).unwrap();
    // TODO: when we never create a window, this doesn't close itself yet.
}*/

pub struct App<'life, C: Composable<'life>> {
    pub composable: Option<C>,
    pub composition: Option<ResponsiblePin<'life, Option<Composition<'life, C>>>>,
}

// ------------------------------
// NOTE:
// This idea won't work, we need ownership of a option (or any other enum/place) in order to initalize it and prevent a double drop in case of a panic.
// This is (at least to my current knowledge) only really possible when using another Option<> wrapper and requiring ownership via take().unwrap().
// The result is an additional overhead for every branch I don't want to take, plus all the extra burden for making pin work, which is a mess to maintain
// and only really "profits" in the rare scenario of needing pinned data, e.g. a future. Those should in future just use Box or something else.
// ------------------------------

impl<'life, C: Composable<'life>> ApplicationHandler for App<'life, C> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        unsafe {
            LOOP.set(Some(core::mem::transmute(event_loop)));
        }
        self.composition = Some(self.composition.take().unwrap().make_some(|inner| unsafe {
            let _ = Composition::open(inner, self.composable.take().unwrap());
        }));
        LOOP.set(None);
    }

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("Suspended! Dropping composition..");
        self.composition = self.composition.make_none();
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

impl<'life, C: Composable<'life>> Drop for App<'life, C> {
    fn drop(&mut self) {
        println!("Dropped! Dropping composition..");
        self.composition = self.composition.make_none();
    }
}
