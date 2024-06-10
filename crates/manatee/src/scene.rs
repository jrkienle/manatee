use std::{iter::once, sync::Arc};

mod scene_manager;
pub use scene_manager::SceneManager;

use crate::ecs::{Component, ComponentManager};
use crate::ecs::{Entity, EntityManager};
use crate::ecs::{System, SystemManager};
use crate::game::Context;
use crate::graphics::{Gpu, RenderTarget};

pub struct Scene {
    pub(crate) components: ComponentManager,
    pub(crate) entities: EntityManager,
    pub(crate) systems: SystemManager,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            components: ComponentManager::new(),
            entities: EntityManager::new(),
            systems: SystemManager::new(),
        }
    }

    pub fn render(&mut self, gpu: Arc<Gpu>) {
        let mut render_target = RenderTarget::new(gpu.clone());
        let mut ctx = Context {
            components: &mut self.components,
            entities: &mut self.entities,
            gpu,
            render_target: &mut render_target,
        };

        for (_, system) in self.systems.systems.iter_mut() {
            system.on_update(&mut ctx);
        }

        ctx.gpu.queue.submit(once(render_target.encoder.finish()));
        render_target.surface_texture.present();
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

impl Default for Scene {
    fn default() -> Self {
        Scene::new()
    }
}
