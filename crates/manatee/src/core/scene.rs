use super::game::GameContext;

pub struct Scene<'a> {
    pub(crate) game: &'a mut GameContext<'a>,
}

impl<'a> Scene<'a> {
    pub fn new(game_context: &'a mut GameContext<'a>) -> Self {
        Self {
            game: game_context
        }
    }
}

pub struct SceneContext {}

impl SceneContext {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SceneContext {
    fn default() -> Self {
        SceneContext::new()
    }
}
