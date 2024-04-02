use crate::{resource::Resources, scope::Scope};

pub struct Context<'scope, 'call> {
    pub resources: &'call mut Resources,
    pub scope: &'scope mut Scope,
}
