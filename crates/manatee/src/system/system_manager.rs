use super::System;
use hashbrown::HashMap;
pub struct SystemManager {
    pub(crate) systems: HashMap<i32, Box<dyn System>>,
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            systems: HashMap::new(),
        }
    }

    pub fn register_system(&mut self, system: impl System + 'static) {
        self.systems.insert(1, Box::new(system));
    }
}
