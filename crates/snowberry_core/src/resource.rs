use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct Resources {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.resources.get(&TypeId::of::<T>()).map(|any| unsafe {
            // SAFETY: Can only be inserted internal and is should always be correct.
            any.downcast_ref().unwrap_unchecked()
        })
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .map(|any| unsafe {
                // SAFETY: Can only be inserted internal and is should always be correct.
                any.downcast_mut().unwrap_unchecked()
            })
    }
}
