use snowberry::core::{Construct, Snowberry};

fn main() {
    Snowberry::new().add_root(content("Hello, world!"));
}

fn content(label: &str) -> impl Construct + '_ {
    move || {
        println!("{label}");
    }
}
