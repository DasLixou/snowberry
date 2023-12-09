use snowberry::core::{Branch, Construct, ConstructFn, Snowberry};

fn main() {
    Snowberry::new().add_root(content("Hello, world!"));
}

fn content(label: &str) -> impl Construct + '_ {
    (move |mut branch: Branch| {
        println!("{label}");
        branch.add_child(child());
    })
    .construct()
}

fn child() -> impl Construct {
    (move || {
        println!("Child :>");
    })
    .construct()
}
