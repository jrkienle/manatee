use crate::{components::Camera, ecs::System};

#[derive(Default)]
pub struct CameraSystem {}

impl System for CameraSystem {
    fn on_update(&self, ctx: &mut crate::Context) {
        let all_camera_instances = ctx.components.get_all_instances::<Camera>();

        // TODO: Figure out if I have to individually downcast_ref here or if I can do this in the
        // ComponentManager
        for (_entity_id, camera_component) in all_camera_instances {
            if camera_component.downcast_ref::<Camera>().unwrap().is_main {
                // TODO: Figure out how to set the main camera's position to its entity's position
                // so I can actually render things at non-global locations in the scene
                // println!("Main Camera Assigned to Entity {entity_id}")
            }
        }
    }
}
