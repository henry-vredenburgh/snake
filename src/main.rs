mod game;
use std::time::{Duration, Instant};

pub const WINDOW_SIZE: usize = 800; // represents 800x800 px screen
pub const MATRIX_SIZE: usize = 10; // represents 10x10 matrix
pub const CELL_SIZE: usize = WINDOW_SIZE / MATRIX_SIZE; // size of each matrix cell in pixels


fn main() {
    let snake_tick = Duration::from_millis(250);
    let mut last_snake_update = Instant::now();

    let mut game = game::Game::new();
    while game.is_running() {
        if last_snake_update.elapsed() >= snake_tick {
            game.render_game();
            game.update();
            last_snake_update = Instant::now();
        }
        game.change_direction();
    }
}
