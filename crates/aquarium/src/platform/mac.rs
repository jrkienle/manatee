use super::Platform;

pub struct MacPlatform {}

impl MacPlatform {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MacPlatform {
    fn default() -> Self {
        MacPlatform::new()
    }
}

impl Platform for MacPlatform {
    fn new_window(&self) {
        println!("TODO: Add Mac Window Open Code");
    }

    fn start(&self) {
        println!("TODO: Add Mac Start Code");
    }
}
