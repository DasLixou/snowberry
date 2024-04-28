use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub use snowberry_core_macros::Resource;

pub trait Resource {}

pub struct Resources {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn with_temp<T: Resource + 'static, F>(&mut self, res: T, f: F)
    where
        F: FnOnce(&mut Self),
    {
        let type_id = TypeId::of::<T>();
        self.resources.insert(type_id, Box::new(res));
        f(self);
        self.resources.remove(&type_id);
    }

    pub fn insert<T: Resource + 'static>(&mut self, val: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(val));
    }

    pub fn get<T: Resource + 'static>(&self) -> Option<&T> {
        self.resources.get(&TypeId::of::<T>()).map(|any| unsafe {
            // SAFETY: Can only be inserted internal and is should always be correct.
            any.downcast_ref().unwrap_unchecked()
        })
    }

    pub fn get_mut<T: Resource + 'static>(&mut self) -> Option<&mut T> {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .map(|any| unsafe {
                // SAFETY: Can only be inserted internal and is should always be correct.
                any.downcast_mut().unwrap_unchecked()
            })
    }
}
