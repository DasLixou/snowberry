use std::convert::Infallible;

pub trait Envy: Copy {}

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

pub trait WithEnv<T> {
    fn get(&self) -> T;
}

impl<T: Envy, Down> Find<T, Down, Infallible> for (Down, T)
where
    Down: MayHave<T>,
{
    type Output = T;

    fn find(&self) -> Self::Output {
        self.1
    }
}

impl<T: Envy, Down, Cur> Find<T, Down, T> for (Down, Cur)
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

/// This is used to get the store generic in the [`composable!`] macro
#[derive(Clone, Copy)]
pub enum EnvTest {}
impl<T> MayHave<T> for EnvTest {
    type Res = T;
    fn retrieve(&self) -> Self::Res {
        unreachable!()
    }
}

// impl Envy for i32 {}
// fn test<Env: WithEnv<i32>>(_env: Env) {
//     test(((), 32i32));
//     test((((), 32i32), 32i32));
//     test((((), 32i32), 32i16));
//     // test((((), 32i8), 32i16));
// }
