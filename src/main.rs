
use crate::game_window::GameWindow;

mod game_window;
mod mesh;
mod graphics;

fn main() {
    let mut game_window = init_game();
    game_window.run_loop();
}

fn init_game() -> GameWindow {
    let mut game_window = GameWindow::new();
    game_window.create_window();
    game_window.init_renderer();
    game_window
}
