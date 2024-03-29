use crate::runner::Runner;

pub struct Context<R: Runner> {
    pub runner_data: R::Data,
}
