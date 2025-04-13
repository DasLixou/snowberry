use std::marker::PhantomData;

pub struct Context<Env> {
    pub env: PhantomData<Env>,
}
