pub struct EventStation<E> {
    listeners: Vec<Box<dyn Fn(&E)>>,
}

impl<E> EventStation<E> {
    pub fn new() -> Self {
        Self { listeners: vec![] }
    }

    pub fn listen<F>(&mut self, f: F)
    where
        F: Fn(&E) + 'static,
    {
        self.listeners.push(Box::new(f));
    }

    pub fn run(&self, event: E) {
        for listener in &self.listeners {
            listener(&event)
        }
    }
}
