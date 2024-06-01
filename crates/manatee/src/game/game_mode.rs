pub struct GameMode {}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::new()
    }
}

impl GameMode {
    pub fn new() -> Self {
        Self {}
    }
}
