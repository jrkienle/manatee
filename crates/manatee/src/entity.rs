mod entity_manager;
pub use entity_manager::EntityManager;

use crate::component::Component;

pub struct Entity {
    // components: Vec<Component>,
    name: &'static str,
}

impl Entity {
    pub fn new(name: &'static str) -> Self {
        println!("Created Entity: {name}!");
        Self { name }
    }

    pub fn add_component<C: Component>(&self, _component: &mut C) {
        println!("Added component!");
    }
}
