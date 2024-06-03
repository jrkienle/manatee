mod scene_manager;
pub use scene_manager::SceneManager;

use crate::component::{Component, ComponentManager};
use crate::entity::{Entity, EntityManager};
use crate::system::{System, SystemManager};

pub struct Scene {
    component_manager: ComponentManager,
    entity_manager: EntityManager,
    system_manager: SystemManager,
}

impl Default for Scene {
    fn default() -> Self {
        Scene::new()
    }
}

impl Scene {
    pub fn new() -> Self {
        Self {
            component_manager: ComponentManager::new(),
            entity_manager: EntityManager::new(),
            system_manager: SystemManager::new(),
        }
    }

    pub fn spawn<C: Component>(&mut self, component: C) -> &Entity {
        let entity = self.entity_manager.add();
        self.component_manager
            .add_component_to_entity(component, entity);
        entity
    }
}
