use std::convert::Infallible;

/// Declares a type as usable for environment down-passing in snowberry.
pub trait Environmentable: Copy {}

/// A trait implemented on any tuple that "may have" type `T` somewhere.
///
/// When it has it, `MayHave::Res` is `T`, otherwise it's `Infallible`. \
/// This is used to not have overlapping implementations,
/// but two distinct sets of either having or not having.
trait MayHave<T> {
    /// Either `Infallible` or `T`
    type Res;

    fn retrieve(&self) -> Self::Res;
}

impl<T> MayHave<T> for () {
    type Res = Infallible;

    fn retrieve(&self) -> Self::Res {
        unreachable!()
    }
}

trait Find<T, Down: MayHave<T>, Res = <Down as MayHave<T>>::Res> {
    type Output;

    fn find(&self) -> Self::Output;
}

/// A trait which is only implemented for a 2-sized left-recursing tuple
/// where `T` is included somewhere.
pub trait WithEnv<T> {
    /// Gets the highest `T` in the tuple stack.
    fn get(&self) -> T;
}

impl<T: Environmentable, Down> Find<T, Down, Infallible> for (Down, T)
where
    Down: MayHave<T>,
{
    type Output = T;

    fn find(&self) -> Self::Output {
        self.1
    }
}

impl<T: Environmentable, Down, Cur> Find<T, Down, T> for (Down, Cur)
where
    Down: MayHave<T>,
{
    type Output = Down::Res;

    fn find(&self) -> Self::Output {
        self.0.retrieve()
    }
}

impl<T, Down, Cur> MayHave<T> for (Down, Cur)
where
    Down: MayHave<T>,
    Self: Find<T, Down>,
{
    type Res = <Self as Find<T, Down>>::Output;

    fn retrieve(&self) -> Self::Res {
        self.find()
    }
}

impl<T, M> WithEnv<T> for M
where
    M: MayHave<T, Res = T>,
{
    fn get(&self) -> T {
        self.retrieve()
    }
}
