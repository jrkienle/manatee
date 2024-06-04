use super::Component;

pub struct Entity {
    pub(crate) id: u32,
}

impl Entity {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn add_component<C: Component>(&self, _component: &mut C) {
        println!("Added component!");
    }
}
