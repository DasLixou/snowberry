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

    pub fn with_temp<T: 'static, F>(&mut self, res: T, f: F)
    where
        F: FnOnce(&mut Self),
    {
        let type_id = TypeId::of::<T>();
        self.resources.insert(type_id, Box::new(res));
        f(self);
        self.resources.remove(&type_id);
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