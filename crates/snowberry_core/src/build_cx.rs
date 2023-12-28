use bumpalo::Bump;
use type_map::TypeMap;

pub struct Context<'g, 's> {
    pub global_resources: &'g mut TypeMap,
    pub(crate) bank: &'s Bump,
}

impl<'g, 's> Context<'g, 's> {
    pub fn deposit<T>(&self, val: T) -> &'s T {
        self.bank.alloc(val)
    }
}
