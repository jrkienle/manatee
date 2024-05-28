pub struct GameObject {
    name: &'static str,
}

impl GameObject {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}
