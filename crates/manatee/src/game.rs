use aquarium::{App, WindowParams};

pub struct Game {
    app: App,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let app = App::new();
        Self { app }
    }

    pub fn run(self) {
        println!("Running your game");
        self.app.run(|app_ctx| {
            app_ctx.new_window(WindowParams {
                title: "Manatee Game",
                ..Default::default()
            });
        })
    }
}
