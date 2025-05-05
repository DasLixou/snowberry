/// Declares a type as usable for environment down-passing in snowberry.
pub trait Environmentable: Copy {
    type Key;
}

/// A trait implemented on any tuple that "may have" type `T` somewhere.
///
/// When it has it, `MayHave::Res` is `T`, otherwise it's `Infallible`. \
/// This is used to not have overlapping implementations,
/// but two distinct sets of either having or not having.
pub trait MayHave<Key> {
    /// Either `Infallible` or `T`
    type Res;

    fn retrieve(&self) -> Self::Res;
}

pub struct Has<T>(T);
pub enum Hasnt {}

impl<Key> MayHave<Key> for () {
    type Res = Hasnt;

    fn retrieve(&self) -> Self::Res {
        unreachable!()
    }
}

pub trait Find<Key, Down: MayHave<Key>, Res = <Down as MayHave<Key>>::Res> {
    type Output;

    fn find(&self) -> Self::Output;
}

/// A trait which is only implemented for a 2-sized left-recursing tuple
/// where `T` is included somewhere.
pub trait WithEnv<T> {
    type Final;

    /// Gets the highest `T` in the tuple stack.
    fn get(&self) -> Self::Final;
}

impl<E: Environmentable, Down> Find<E::Key, Down, Hasnt> for (Down, E)
where
    Down: MayHave<E::Key>,
{
    type Output = Has<E>;

    fn find(&self) -> Self::Output {
        Has(self.1)
    }
}

impl<E: Environmentable, Down, Cur> Find<E::Key, Down, Has<E>> for (Down, Cur)
where
    Down: MayHave<E::Key>,
{
    type Output = Down::Res;

    fn find(&self) -> Self::Output {
        self.0.retrieve()
    }
}

impl<Key, Down, Cur> MayHave<Key> for (Down, Cur)
where
    Down: MayHave<Key>,
    Self: Find<Key, Down>,
{
    type Res = <Self as Find<Key, Down>>::Output;

    fn retrieve(&self) -> Self::Res {
        self.find()
    }
}

// impl<E: Environmentable, M> WithEnv<E::Key> for M
// where
//     M: MayHave<E, Res = Has<E>>,
// {
//     fn get(&self) -> E {
//         self.retrieve().0
//     }
// }

// TODO: filter out Hasnt things (prob with a second assoc type in MayHave)
impl<Key, M> WithEnv<Key> for M
where
    M: MayHave<Key /* , Res = Has<E>*/>,
{
    type Final = M::Res;

    fn get(&self) -> Self::Final {
        self.retrieve()
    }
}
