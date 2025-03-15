use std::convert::Infallible;

trait Envy {}
impl Envy for i32 {}

trait MayHave<T> {
    /// Either `Infallible` or `T`
    type Res;
}

impl<T> MayHave<T> for () {
    type Res = Infallible;
}

trait Find<T, Down: MayHave<T>, Res = <Down as MayHave<T>>::Res> {
    type Output;
}

trait WithEnv<T> {}

impl<T: Envy, Down> Find<T, Down, Infallible> for (Down, T)
where
    Down: MayHave<T>,
{
    type Output = T;
}

impl<T: Envy, Down, Cur> Find<T, Down, T> for (Down, Cur)
where
    Down: MayHave<T>,
{
    type Output = Down::Res;
}

impl<T, Down, Cur> MayHave<T> for (Down, Cur)
where
    Down: MayHave<T>,
    Self: Find<T, Down>,
{
    type Res = <Self as Find<T, Down>>::Output;
}

impl<T, M> WithEnv<T> for M where M: MayHave<T, Res = T> {}

fn test<Env: WithEnv<i32>>(_env: Env) {
    test(((), 32i32));
    test((((), 32i32), 32i32));
    test((((), 32i32), 32i16));
    // test((((), 32i8), 32i16));
}
