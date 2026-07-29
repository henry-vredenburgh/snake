mod game;
mod paint;
pub const LENGTH: usize = 800;
pub const BORDER: usize = 100;
pub const BLOB_SIZE: usize = 10;

use std::time::{Duration, Instant};


fn main() {

    let snake_tick = Duration::from_millis(25);
    let mut last_snake_update = Instant::now();
    let mut game = game::Game::new();
    game.init_game();

    while game.is_running() {
        if last_snake_update.elapsed() >= snake_tick {
            game.update();
            last_snake_update = Instant::now();
        }
        game.read_inputs_and_change_direction();
    }
}
