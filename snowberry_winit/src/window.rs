use snowberry::{
    composable, composable::Composable, composition::Composition, lens_table::LensTable,
};
use winit::window::WindowAttributes;

use crate::runner::LOOP;

pub fn window<'life, D: Copy + 'life>(
    attributes: WindowAttributes,
    child: impl Composable<'life, Down = LensTable<D>>,
) -> impl Composable<'life, Down = D> {
    composable!(|cx| {
        // TODO: handle open and closing correctly with creating and dropping store of child in Option
        let window = LOOP.get().unwrap().create_window(attributes).unwrap();
        let cx = cx.store(window);
        // TODO: provide access to window down the tree
        let (cx, comp) = Composition::open(cx, child);
        let cx = cx.store(comp);
        cx
    })
}
