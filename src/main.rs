use crate::game_window::GameWindow;

mod game_window;
mod mesh;
mod graphics;
mod assets;
mod registry;
mod game;

fn main() {
    let mut game_window = GameWindow::new();
    game_window.run_loop();
}
