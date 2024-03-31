use crate::{runner::Runner, scope::Scope};

pub struct Context<'scope, R: Runner> {
    pub runner_data: R::Data,
    pub scope: &'scope mut Scope,
}
