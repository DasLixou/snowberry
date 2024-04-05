use std::error::Error;

use snowberry::core::{app::App, context::Context};
use snowberry::winit::{window, WinitRunner};

fn main() -> Result<(), Box<dyn Error>> {
    App::new().run(WinitRunner, content)
}

fn content(cx: &mut Context<'_, '_>) {
    window(cx, "My Snowberry UI", |cx: &mut Context<'_, '_>| {
        println!("I am in the main window");
        cx.store(TextBomb("MAIN WINDOW KABOOM!"))
    });
    window(cx, "Another Window", |cx: &mut Context<'_, '_>| {
        println!("this is another window");
        cx.store(TextBomb("pew pew"));
    });
}

pub struct TextBomb(&'static str);

impl Drop for TextBomb {
    fn drop(&mut self) {
        println!("TextBomb dropped '{}'", self.0);
    }
}
