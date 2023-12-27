use crate::Snowberry;

pub trait Runner: Sized {
    fn run(self, snowberry: Snowberry);
}
