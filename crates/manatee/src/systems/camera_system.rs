use crate::{
    components::{Camera, Transform},
    ecs::System,
    game::Context,
};

#[derive(Default)]
pub struct CameraSystem {}

impl System for CameraSystem {
    fn on_update(&self, ctx: &mut Context) {
        let all_cameras = ctx.components.get_all::<Camera>();
        let mut main_camera_entity_id: Option<&u32> = None;

        if all_cameras.is_some() {
            for (entity_id, camera_component) in all_cameras.unwrap() {
                if camera_component.downcast_ref::<Camera>().unwrap().is_main {
                    main_camera_entity_id = Some(entity_id);
                }
            }
        }

        if main_camera_entity_id.is_some() {
            let _camera_position = ctx
                .components
                .get::<Transform>(&main_camera_entity_id.unwrap())
                .expect("Entity with main Camera component missing required Transform component");
        }
    }
}
