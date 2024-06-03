use super::Component;
use crate::entity::Entity;
use hashbrown::HashMap;
use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
};

pub struct ComponentManager {
    pub(crate) components: HashMap<String, HashMap<u32, Box<RefCell<dyn Component>>>>,
    // pub(crate) component_types: Vec<String>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn add_component_to_entity<C: Component>(&mut self, component: C, entity: &Entity) {
        let entity_id = entity.id;
        let component_name = component.type_name().to_string();
        println!("Added Component with Name {component_name} to {entity_id}");

        let component_row = self
            .components
            .entry(component_name)
            .or_insert(HashMap::new());

        component_row.insert(entity_id, Box::new(RefCell::new(component)));
        // let entities = vec![entity_id];
        // self.components.insert(0, entities);
        // let component_name = component.type_name();
    }
}
