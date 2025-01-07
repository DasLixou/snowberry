use std::{
    cell::Cell,
    mem::MaybeUninit,
    pin::{pin, Pin},
};

use snowberry::{composable::Composable, composition::Composition};
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

        //::snowberry::responsible_pin!(let unsafe composition = ::std::mem::MaybeUninit::uninit());
        let composition = ::std::pin::pin!(::std::mem::MaybeUninit::uninit());

        let mut app = $crate::runner::App {
            composable: Some($($tokens)*),
            composition: Some(composition),
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
    pub composition: Option<Pin<&'life mut MaybeUninit<Composition<'life, C>>>>,
}

impl<'life, C: Composable<'life>> ApplicationHandler for App<'life, C> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        unsafe {
            LOOP.set(Some(core::mem::transmute(event_loop)));
        }
        self.composition = Some(unsafe {
            Composition::open(
                self.composition.take().unwrap(),
                self.composable.take().unwrap(),
            )
        });
        LOOP.set(None);
    }

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(comp) = self.composition.take() {
            println!("Suspended! Dropping composition..");
            unsafe {
                comp.get_unchecked_mut().assume_init_drop();
            }
        }
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
        if let Some(comp) = self.composition.take() {
            println!("Dropped! Dropping composition..");
            unsafe {
                comp.get_unchecked_mut().assume_init_drop();
            }
        }
    }
}
