use super::{Component, Entity};
use hashbrown::HashMap;

pub struct ComponentManager {
    pub(crate) components: HashMap<String, HashMap<u32, Box<dyn Component>>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn add<C: Component>(&mut self, component: C, entity: &Entity) {
        let entity_id = entity.id;
        let component_name = component.type_name().to_string();
        println!("Added Component {component_name} to Entity {entity_id}");

        let component_row = self.components.entry(component_name).or_default();

        component_row.insert(entity_id, Box::new(component));
    }

    // TODO: Figure out how the fuck to cast the output type to the generic, is that even a thing
    // that Rust lets me do??? I miss TypeScript sometimes
    pub fn get_all<C: Component>(&self) -> Option<&HashMap<u32, Box<dyn Component>>> {
        // This is probably really slow, I gotta think of a way to make this better
        let component_name = std::any::type_name::<C>().to_string();
        self.components.get(&component_name)
    }

    pub fn get<C: Component>(&self, entity_id: &u32) -> Option<&Box<dyn Component>> {
        // This is probably really slow, I gotta think of a way to make this better
        let all_instances = self.get_all::<C>();
        if all_instances.is_none() {
            return None;
        }
        all_instances.unwrap().get(entity_id)
    }
}

impl Default for ComponentManager {
    fn default() -> Self {
        ComponentManager::new()
    }
}
