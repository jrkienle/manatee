mod scene_manager;
pub use scene_manager::SceneManager;

use crate::entity::EntityManager;
use crate::system::{System, SystemManager};

pub struct Scene {
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
            entity_manager: EntityManager::new(),
            system_manager: SystemManager::new(),
        }
    }

    pub fn register_system(&mut self, system: impl System + 'static) {
        self.system_manager.register_system(system);
    }

    pub fn get_system(self, id: i32) -> System {
        let foo = self
            .system_manager
            .systems
            .get(&1)
            .expect("Invalid system ID");
    }
}
