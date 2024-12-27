use std::{mem::MaybeUninit, pin::Pin, ptr::drop_in_place};

use snowberry::{composable, composable::Composable, composition::Composition};
use winit::window::Window;

use crate::runner::LOOP;

pub fn window<'life, C: Composable<'life>>(title: String, child: C) -> impl Composable<'life> {
    composable! {
        |store| {
            // TODO: handle open and closing correctly with creating and dropping store of child in Option
            let window = Window::default_attributes().with_title(title);
            let window = LOOP.get().unwrap().create_window(window).unwrap();
            let (store, _) = store.store_val(window);
            let (store, window_child) = store.store_val(WindowChild::<'life, C> { mem: Some(MaybeUninit::uninit()) });
            unsafe {
                // TODO: lifetime shouldn't be the same but smaller
                let wc = window_child.get_unchecked_mut();
                let mem = Pin::new_unchecked(wc.mem());
                Composition::open(mem, child);
            }
            store.end();
        }
    }

    /*struct WindowComposable<'life, C>(String, PhantomData<&'life C>);
    impl<'life, C: Composable<'life>> Composable<'life> for WindowComposable<'life, C> {
        type Store = WindowChild<'life, C>;
        fn compose(self, _store: snowberry::store::Store<'life, Self::Store>) {
            // TODO: handle open and closing correctly with creating and dropping store of child in Option
            let window = Window::default_attributes().with_title(self.0);
            let window = LOOP.get().unwrap().create_window(window).unwrap();
        }
    }
    WindowComposable::<C>(title, PhantomData)*/
}

struct WindowChild<'life, C: Composable<'life>> {
    // TODO: lifetime shouldn't be the same but smaller
    mem: Option<MaybeUninit<Composition<'life, C>>>,
}

impl<'life, C: Composable<'life>> WindowChild<'life, C> {
    fn mem(&mut self) -> &mut MaybeUninit<Composition<'life, C>> {
        self.mem.as_mut().unwrap()
    }
}

impl<'life, C: Composable<'life>> Drop for WindowChild<'life, C> {
    fn drop(&mut self) {
        if let Some(mut init) = self.mem.take() {
            unsafe {
                drop_in_place(init.as_mut_ptr());
            }
        }
    }
}
