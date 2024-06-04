use super::System;
use hashbrown::HashMap;
use std::cell::RefCell;

pub struct SystemManager {
    pub(crate) systems: HashMap<u32, Box<RefCell<dyn System>>>,
    len: u32,
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            len: 0,
            systems: HashMap::new(),
        }
    }

    pub fn register_system<S: System>(&mut self, system: S) -> &RefCell<dyn System> {
        let system_id = self.len;
        self.systems
            .insert(system_id, Box::new(RefCell::new(system)));
        self.len += 1;
        println!("Added entity with ID {system_id}");
        self.systems.get(&system_id).unwrap().as_ref()
    }
}
