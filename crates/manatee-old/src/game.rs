// mod asset;
// mod context;
// mod game_mode;
// mod window_state;

// use winit::event_loop::EventLoop;

// use crate::{scene::Scene, scene::SceneManager};

// pub use asset::Asset;
// pub use context::Context;
// pub use game_mode::GameMode;
// pub use window_state::WindowState;

// pub struct Game {
//     event_loop: EventLoop<()>,
//     window_state: WindowState,
// }

// impl Game {
//     pub fn new() -> Self {
//         let event_loop = EventLoop::new().unwrap();
//         let window_state = WindowState::new();
//         Self {
//             event_loop,
//             window_state,
//         }
//     }

//     pub fn load_scene(&mut self, scene: Scene) {
//         self.window_state.scene_manager.load_scene(scene);
//     }

//     pub fn run(mut self) {
//         self.event_loop.run_app(&mut self.window_state).unwrap();
//     }
// }

// impl Default for Game {
//     fn default() -> Self {
//         Self::new()
//     }
// }

mod context;
mod game_mode;


use crate::{scene::Scene, scene::SceneManager};
use aquarium::{App, WindowParams};

pub use context::Context;
pub use game_mode::GameMode;
// pub use window_state::WindowState;

pub struct Game {
    app: App,
    // event_loop: EventLoop<()>,
    // window_state: WindowState,
}

impl Game {
    pub fn new() -> Self {
        let app = App::new();
        Self {
            app
        }
        // let event_loop = EventLoop::new().unwrap();
        // let window_state = WindowState::new();
        // Self {
        //     event_loop,
        //     window_state,
        // }
    }

    pub fn load_scene(&mut self, scene: Scene) {
        // self.window_state.scene_manager.load_scene(scene);
    }

    pub fn run(self) {
        // self.event_loop.run_app(&mut self.window_state).unwrap();
        self.app.run(|ctx| {
            ctx.new_window(WindowParams { title: "Manatee Game" })
        })
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
