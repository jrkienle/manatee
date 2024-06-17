use crate::{
    components::{Camera, Transform},
    ecs::System,
    game::Context,
};

#[derive(Default)]
pub struct CameraSystem {}

impl System for CameraSystem {
    fn on_update(&self, ctx: &mut Context) {
        let all_camera_instances = ctx.components.get_all_instances::<Camera>();
        let mut main_camera_entity_id: Option<&u32> = None;

        if all_camera_instances.is_some() {
            for (entity_id, camera_component) in all_camera_instances.unwrap() {
                if camera_component.downcast_ref::<Camera>().unwrap().is_main {
                    main_camera_entity_id = Some(entity_id);
                    // TODO: Figure out how to set the main camera's position to its entity's position
                    // so I can actually render things at non-global locations in the scene
                    // println!("Main Camera Assigned to Entity {entity_id}")
                }
            }
        }

        if main_camera_entity_id.is_some() {
            println!("Main Camera Found");
            let _camera_position = ctx
                .components
                .get_instance::<Transform>(&main_camera_entity_id.unwrap())
                .expect("Entity with main Camera component missing required Transform component");
        }
    }
}
