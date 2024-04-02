use std::error::Error;

use snowberry::core::{app::App, context::Context};
use snowberry::window::{window, WinitRunner};

fn main() -> Result<(), Box<dyn Error>> {
    App::new().run(WinitRunner, content)
}

fn content(cx: Context<'_, '_>) {
    window(cx, "My Snowberry UI", |_cx: Context<'_, '_>| {
        println!("hey");
    })
}
