use std::sync::Arc;

use crate::{ecs::{ComponentManager, EntityManager}, graphics::{Gpu,RenderTarget}};
pub struct Context<'a> {
    pub components: &'a mut ComponentManager,
    pub entities: &'a mut EntityManager,
    pub gpu: Arc<Gpu>,
    pub render_target: &'a mut RenderTarget,
}
