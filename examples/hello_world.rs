use std::cell::RefCell;
use std::error::Error;

use snowberry::core::{app::App, context::Context};
use snowberry::vello::{self, Scene};
use snowberry::winit::{window, WinitRunner};
use vello::kurbo::{Affine, Rect};
use vello::peniko::Color;

fn main() -> Result<(), Box<dyn Error>> {
    App::new().run(WinitRunner, content)
}

fn content(cx: &mut Context<'_, '_>) {
    vello::init(cx);

    window(cx, "My Snowberry UI", |cx, window| {
        println!("I am in the main window");
        cx.store(TextBomb("MAIN WINDOW KABOOM!"));

        let surface = vello::create_surface(
            cx,
            window,
            window.inner_size().width,
            window.inner_size().height,
        )
        .unwrap();
        vello::prepare_renderer(cx, &surface);

        let mut scene = Scene::new();
        scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            Color::CHARTREUSE,
            None,
            &Rect::new(0., 0., 20., 50.),
        );
        vello::render(cx, &scene, &surface);
    });
    window(cx, "Another Window", |cx, _window| {
        println!("this is another window");
        cx.store(TextBomb("pew pew"));

        let val = cx.store(RefCell::new(41));
        *val.borrow_mut() += 1;
        println!("The magic number is {}", *val.borrow());
    });
}

pub struct TextBomb(&'static str);

impl Drop for TextBomb {
    fn drop(&mut self) {
        println!("TextBomb dropped '{}'", self.0);
    }
}
