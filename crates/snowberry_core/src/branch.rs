use crate::Construct;

pub struct Branch {}

impl Branch {
    pub fn add_child(&mut self, child: impl Construct) {
        child.build();
    }
}
