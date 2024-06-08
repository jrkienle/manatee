use std::sync::Arc;

use super::{render_target::RenderTarget, Gpu};
use crate::ecs::{ComponentManager, EntityManager};
pub struct Context<'a> {
    pub components: &'a mut ComponentManager,
    pub entities: &'a mut EntityManager,
    pub gpu: Arc<Gpu>,
    pub render_target: &'a mut RenderTarget,
}

// impl<'a> Context<'a> {
//     pub fn new(
//         entities: &'a mut EntityManager,
//         components: &'a mut ComponentManager,
//         gpu: Arc<Gpu>,
//     ) -> Self {
//         Self {
//             components,
//             entities,
//             gpu,
//         }
//     }
// }
