use snowberry::core::{Branch, Constructable, Snowberry};

fn main() {
    Snowberry::new().add_root(content.construct("Hello, world!"));
}

fn content(label: &str, mut branch: Branch) {
    println!("{label}");
    branch.add_child(child.construct(()));
}

fn child(_: (), _: ()) {
    println!("Child :>")
}
