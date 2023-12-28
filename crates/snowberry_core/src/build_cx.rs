use type_map::TypeMap;

pub struct Context<'s> {
    pub global_resources: &'s mut TypeMap,
}
