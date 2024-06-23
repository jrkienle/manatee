pub struct Window {}

pub struct WindowParams {
    pub title: &'static str,
}

impl Default for WindowParams {
    fn default() -> Self {
        Self {
            title: "Aquarium Window"
        }
    }
}
