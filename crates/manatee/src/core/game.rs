use super::scene::Scene;
use aquarium::{App, AppContext, WindowParams};
use hashbrown::HashMap;

pub struct Game {
    title: &'static str,
}

impl Game {
    pub fn new(title: &'static str) -> Self {
        Self { title }
    }

    pub fn run<F>(self, on_start: F)
    where
        F: 'static + FnOnce(&mut GameContext),
    {
        App::new().run(move |app| {
            {
                app.new_window(WindowParams { title: self.title })
            }

            {
                let mut game_context = GameContext::new(app);
                on_start(&mut game_context)
            }
            
    })
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new("Manatee Game")
    }
}

pub struct GameContext<'a> {
    active_scene: Option<u32>,
    scenes_length: u32,
    scenes: HashMap<u32, Scene<'a>>,
    windows: &'a mut AppContext,
}

impl<'a> GameContext<'a> {
    pub fn new(app_context: &'a mut AppContext) -> Self {
        Self {
            active_scene: None,
            scenes_length: 0,
            scenes: HashMap::new(),
            windows: app_context,
        }
    }

    pub fn new_scene(&'a mut self) -> Scene<'a> {
        Scene::new(self)
    }
}
