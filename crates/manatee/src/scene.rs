mod scene_manager;
pub use scene_manager::SceneManager;

use crate::ecs::{Component, ComponentManager};
use crate::ecs::{Entity, EntityManager};
use crate::ecs::{System, SystemManager};

pub struct Scene {
    pub(crate) components: ComponentManager,
    pub(crate) entities: EntityManager,
    pub(crate) systems: SystemManager,
}

impl Default for Scene {
    fn default() -> Self {
        Scene::new()
    }
}

impl Scene {
    pub fn new() -> Self {
        Self {
            components: ComponentManager::new(),
            entities: EntityManager::new(),
            systems: SystemManager::new(),
        }
    }

    pub fn spawn<C: Component>(&mut self, component: C) -> &Entity {
        let entity = self.entities.add();
        self.components.add_component_to_entity(component, entity);
        entity
    }

    pub fn register_system<S: System>(&mut self, system: S) {
        self.systems.register_system(system);
    }
}
