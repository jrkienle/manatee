#[derive(Default)]
pub struct Game {}

impl Game {
    pub fn start<F>(self, on_start: F)
    where
        F: FnOnce(),
    {
        on_start();
    }
}
