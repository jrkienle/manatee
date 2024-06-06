use super::WindowState;
use crate::{
    ecs::{ComponentManager, EntityManager, SystemManager},
    scene::SceneManager,
};
pub struct Context<'a> {
    pub components: &'a mut ComponentManager,
    pub entities: &'a mut EntityManager,
    pub game: &'a mut WindowState,
}

// impl<'a> Context<'a> {
//     pub fn new(window_state: &'a mut WindowState) -> (&'a mut SystemManager, Self) {
//         let game = window_state;
//         let active_scene = game.scene_manager.active_scene();
//         (
//             &mut active_scene.systems,
//             Self {
//                 components: &mut active_scene.components,
//                 entities: &mut active_scene.entities,
//                 game,
//             },
//         )
//     }
// }
