use snowberry::{composable, composable::Composable, composition::Composition};
use winit::window::WindowAttributes;

use crate::runner::LOOP;

pub fn window<'life>(
    attributes: WindowAttributes,
    child: impl Composable<'life>,
) -> impl Composable<'life> {
    composable!(|cx| {
        // TODO: handle open and closing correctly with creating and dropping store of child in Option
        let window = LOOP.get().unwrap().create_window(attributes).unwrap();
        let cx = cx.store(window);
        // TODO: provide access to window down the tree
        let cx = cx.store(Composition::open(child));
        cx
    })
}
