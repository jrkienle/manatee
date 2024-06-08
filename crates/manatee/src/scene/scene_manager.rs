use crate::scene::Scene;

pub struct SceneManager {
    pub(crate) active_scene: Option<Scene>,
}

impl Default for SceneManager {
    fn default() -> Self {
        SceneManager::new()
    }
}

impl SceneManager {
    pub fn new() -> Self {
        Self { active_scene: None }
    }

    pub fn load_scene(&mut self, scene: Scene) {
        self.active_scene = Some(scene);
    }
}
