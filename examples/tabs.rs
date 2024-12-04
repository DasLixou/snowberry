use snowberry::{composable::Composable, composition::Composition};

fn main() {
    Composition::open(|store: _| {
        let (store, branch) = store.store_val(Cell::new(Tabs::Text(text())));
        store.end();
    });
}

enum Tabs<'scope, A, B> {
    A(Composition<'scope, A>),
    B(Composition<'scope, B>),
}

fn a<'s>() -> impl Composable<'s, /* PROBLEM: WE CAN'T INFER HERE :c */ ()> {
    |store: _| {
        store.end();
    }
}

const B: impl Composable = |store: _| {
    store.end();
};
