pub struct Window {}

pub struct WindowParams {
    pub height: u16,
    pub title: &'static str,
    pub width: u16,
}

impl Default for WindowParams {
    fn default() -> Self {
        Self {
            height: 600,
            title: "Aquarium Window",
            width: 800,
        }
    }
}
