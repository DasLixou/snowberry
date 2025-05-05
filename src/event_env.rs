//! This is a clone of environment.rs, but holds (T, impl Fn()) instead of just T, so that we can get an annonymous function from a staticly known type.

pub struct EEHas;
pub struct EEHasnt;

pub trait EEMayHave<E> {
    type Status;

    fn retrieve(&self) -> impl Fn();
}

impl<E> EEMayHave<E> for () {
    type Status = EEHasnt;

    fn retrieve(&self) -> impl Fn() {
        || unreachable!()
    }
}

pub trait EEFind<E, Down: EEMayHave<E>, Status = <Down as EEMayHave<E>>::Status> {
    type Status;

    fn find(&self) -> impl Fn();
}

pub trait WithCallback<E> {
    fn get(&self) -> impl Fn();
}

impl<E: Copy, F: Fn() + Copy, Down> EEFind<E, Down, EEHasnt> for (Down, (E, F))
where
    Down: EEMayHave<E>,
{
    type Status = EEHas;

    fn find(&self) -> impl Fn() {
        self.1.1
    }
}

impl<E, Down, Cur> EEFind<E, Down, EEHas> for (Down, Cur)
where
    Down: EEMayHave<E>,
{
    type Status = Down::Status;

    fn find(&self) -> impl Fn() {
        self.0.retrieve()
    }
}

impl<E, Down, Cur> EEMayHave<E> for (Down, Cur)
where
    Down: EEMayHave<E>,
    Self: EEFind<E, Down>,
{
    type Status = <Self as EEFind<E, Down>>::Status;

    fn retrieve(&self) -> impl Fn() {
        self.find()
    }
}

impl<E, M> WithCallback<E> for M
where
    M: EEMayHave<E, Status = EEHas>,
{
    fn get(&self) -> impl Fn() {
        self.retrieve()
    }
}
