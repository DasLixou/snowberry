use snowberry::{composable, composable_::Composable, environment::WithEnv, lens_table::LensTable};
use winit::window::WindowAttributes;

use crate::runner::WinitLoop;

pub fn window<'life, D: Copy + 'life, E: Copy + 'life>(
    attributes: WindowAttributes,
    child: impl Composable<'life, Down = LensTable<D>, Env = E>,
) -> impl Composable<'life, Down = D, Env = E>
where
    E: WithEnv<WinitLoop>,
{
    composable!(|cx| {
        // TODO: handle open and closing correctly with creating and dropping store of child in Option
        let window = cx.env::<WinitLoop>().0.create_window(attributes).unwrap();
        let (cx, _) = cx.store(window);
        // TODO: provide access to window down the tree
        let cx = cx.compose(child);
        cx
    })
}
