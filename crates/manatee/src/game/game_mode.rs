pub struct GameMode {}

impl GameMode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::new()
    }
}
