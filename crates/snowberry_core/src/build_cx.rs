use type_map::TypeMap;

pub struct BuildContext<'s> {
    pub global_resources: &'s mut TypeMap,
}
