use crate::{resource::Resources, scope::Scope};

pub struct Context<'scope> {
    pub resources: &'scope mut Resources,
    pub scope: &'scope mut Scope,
}
