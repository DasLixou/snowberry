use std::cell::RefCell;
use std::error::Error;

use snowberry::core::{app::App, context::Context};
use snowberry::vello;
use snowberry::winit::{window, WinitRunner};

fn main() -> Result<(), Box<dyn Error>> {
    App::new().run(WinitRunner, content)
}

fn content(cx: &mut Context<'_, '_>) {
    vello::init(cx);

    window(
        cx,
        "My Snowberry UI",
        |cx: &mut Context<'_, '_>, window: &'_ _| {
            println!("I am in the main window");
            cx.store(TextBomb("MAIN WINDOW KABOOM!"));

            let surface = vello::create_surface(cx, window);
        },
    );
    window(cx, "Another Window", |cx: &mut Context<'_, '_>, _window| {
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
