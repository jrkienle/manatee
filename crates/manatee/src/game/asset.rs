use std::fs::File;

pub trait Asset {
    fn decode(&mut self, file: File);
    fn load(&mut self, path: String) {
        File::open(path).expect("Failed to load file with path `path`");
    }
}