/// Declares a type as usable for environment down-passing in snowberry.
pub trait Environmentable: Copy {}

/// A trait implemented on any tuple that "may have" type `T` somewhere.
///
/// When it has it, `MayHave::Res` is `T`, otherwise it's `Infallible`. \
/// This is used to not have overlapping implementations,
/// but two distinct sets of either having or not having.
pub trait MayHave<E> {
    /// Either `Infallible` or `T`
    type Res;

    fn retrieve(&self) -> Self::Res;
}

pub struct Has<T>(pub T);
pub enum Hasnt {}

impl<E> MayHave<E> for () {
    type Res = Hasnt;

    fn retrieve(&self) -> Self::Res {
        unreachable!()
    }
}

pub trait Find<E, Down: MayHave<E>, Res = <Down as MayHave<E>>::Res> {
    type Output;

    fn find(&self) -> Self::Output;
}

/// A trait which is only implemented for a 2-sized left-recursing tuple
/// where `T` is included somewhere.
pub trait WithEnv<E> {
    type Final;

    /// Gets the highest `T` in the tuple stack.
    fn get(&self) -> Self::Final;
}

impl<E: Environmentable, Down> Find<E, Down, Hasnt> for (Down, E)
where
    Down: MayHave<E>,
{
    type Output = Has<E>;

    fn find(&self) -> Self::Output {
        Has(self.1)
    }
}

impl<E: Environmentable, Down, Cur> Find<E, Down, Has<E>> for (Down, Cur)
where
    Down: MayHave<E>,
{
    type Output = Down::Res;

    fn find(&self) -> Self::Output {
        self.0.retrieve()
    }
}

impl<E, Down, Cur> MayHave<E> for (Down, Cur)
where
    Down: MayHave<E>,
    Self: Find<E, Down>,
{
    type Res = <Self as Find<E, Down>>::Output;

    fn retrieve(&self) -> Self::Res {
        self.find()
    }
}

impl<E, M> WithEnv<E> for M
where
    M: MayHave<E, Res = Has<E>>,
{
    type Final = M::Res;

    fn get(&self) -> Self::Final {
        self.retrieve()
    }
}
