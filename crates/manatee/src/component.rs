pub mod components;

pub trait Component {
    fn hello_world() {
        println!("Hello, world!");
    }
}
