use std::error::Error;

use snowberry::core::{app::App, context::Context};
use snowberry::window::{window, WinitRunner};

fn main() -> Result<(), Box<dyn Error>> {
    App::new().run(WinitRunner, content)
}

fn content(cx: Context<'_, '_, WinitRunner>) {
    window(
        cx,
        "My Snowberry UI",
        |_ctx: Context<'_, '_, WinitRunner>| {
            println!("hey");
        },
    )
}
