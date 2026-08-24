mod game;
use std::time::{Duration, Instant};

pub const WINDOW_SIZE: usize = 1200; // represents 800x800 px screen
pub const MATRIX_SIZE: usize = 10; // represents 10x10 matrix
pub const CELL_SIZE: usize = WINDOW_SIZE / MATRIX_SIZE; // size of each matrix cell in pixels


fn main() {
    let tick: Duration = Duration::from_millis(110);
    let mut last_update: Instant = Instant::now();
    let mut game: game::Game = game::Game::new();
    
    game.render_game();
    game.wait();

    while game.is_running() {
        game.change_direction();
        if last_update.elapsed() >= tick {
            game.update();
            last_update = Instant::now();
        }
        game.render_game();
    }
    let score: u8 = game.get_score();
    drop(game);
    println!("Game Over! You ate {} apples", score);
}