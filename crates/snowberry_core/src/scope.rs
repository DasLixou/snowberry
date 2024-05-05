use std::{cell::RefCell, marker::PhantomData};

use bumpalo::Bump;
use slotmap::new_key_type;

new_key_type! {
    pub struct ScopeKey;
}

#[derive(Clone, Copy)]
pub struct ScopeLife<'scope>(pub PhantomData<&'scope ()>);

pub struct Scope {
    pub store: ScopeStore,
    pub sub_scopes: Vec<ScopeKey>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            store: ScopeStore {
                store: Bump::new(),
                drops: RefCell::new(Vec::new()),
            },
            sub_scopes: Vec::new(),
        }
    }
}

pub struct ScopeStore {
    store: Bump,
    drops: RefCell<Vec<StoreDropper>>,
}

impl ScopeStore {
    pub fn store<'scope, T: 'scope>(&self, _life: ScopeLife<'scope>, val: T) -> &'scope T {
        let b = self.store.alloc(val);
        if std::mem::needs_drop::<T>() {
            self.drops.borrow_mut().push(StoreDropper::new(b as *mut T));
        }
        unsafe {
            // TODO: think about safety
            core::mem::transmute(b)
        }
    }
}

struct StoreDropper {
    pointer: *mut (),
    drop_fn: fn(*mut ()),
}

impl StoreDropper {
    pub fn new<T>(pointer: *mut T) -> Self {
        StoreDropper {
            pointer: pointer as *mut (),
            drop_fn: erased_drop::<T>,
        }
    }
}

impl Drop for StoreDropper {
    fn drop(&mut self) {
        (self.drop_fn)(self.pointer)
    }
}

fn erased_drop<T>(erased_ptr: *mut ()) {
    unsafe { core::ptr::drop_in_place(erased_ptr.cast::<T>()) }
}
