use snowberry::{composable, composable::Composable, composition::Composition};

fn main() {
    let bomb = Bomb("static");
    snowberry::run_simple!(simple());
    let _ = bomb;
}

fn simple<'l>() -> impl Composable<'l> {
    composable!(|cx| {
        let cx = cx.store(Bomb("42"));
        let cx = cx.store(Bomb("another one :3"));
        let cx = cx.store(Composition::open(window()));
        cx
    })
}

fn window<'l>() -> impl Composable<'l> {
    composable!(|cx| {
        let cx = cx.store(Bomb("..and the window second :>"));
        let cx = cx.store(Bomb("Uninitialize render cx first.."));
        cx
    })
}

struct Bomb(&'static str);
impl Drop for Bomb {
    fn drop(&mut self) {
        println!("kapow '{}'", self.0);
    }
}
