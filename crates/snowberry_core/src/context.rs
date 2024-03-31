use crate::{runner::Runner, scope::Scope};

pub struct Context<'scope, 'data, R: Runner> {
    pub runner_data: R::Data<'data>,
    pub scope: &'scope mut Scope,
}
