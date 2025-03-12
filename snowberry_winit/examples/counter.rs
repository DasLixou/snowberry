use std::cell::Cell;

use snowberry::{composable, composable_::Composable};
use snowberry_winit::{run_winit, window::window, winit::window::Window};

fn main() {
    let a = Bomb("Heloo");
    run_winit!(counter());
    let _ = a;
}

fn counter<'l, D: Copy + 'l>() -> impl Composable<'l, Down = D> {
    window(
        Window::default_attributes().with_title("my snowberry counter :3"),
        composable!(|cx| {
            let (cx, _) = cx.store(Cell::new(0));

            let (cx, _) = cx.store(Bomb("root!"));

            let cx = cx.compose(button("-"));
            let cx = cx.compose(button("+"));

            let cx = cx.compose(window(
                Window::default_attributes().with_title("subwindow"),
                composable!(|cx| { cx }),
            ));

            // println!("Currently {}", count.get());
            cx
        }),
    )
}

fn button<'l, D: Copy + 'l>(label: &'l str) -> impl Composable<'l, Down = D> {
    composable!(move |cx| {
        println!("Button '{label}'");
        let (cx, _) = cx.store(Bomb("a button :3"));
        cx
    })
}

struct Bomb(&'static str);
impl Drop for Bomb {
    fn drop(&mut self) {
        println!("kapow '{}'", self.0);
    }
}
