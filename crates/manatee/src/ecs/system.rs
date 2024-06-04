use crate::game::Context;

pub trait System: Send + Sync + 'static {
    fn on_update(&self, ctx: &mut Context) {
        println!("Default System Update Ran!")
    }
}
