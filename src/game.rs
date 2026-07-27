#![allow(unused)]

use minifb::{Key, KeyRepeat};
use minifb::{Key::P, Window, WindowOptions};
use rand::RngExt;
use crate::LENGTH;
use crate::paint::{paint_blob, Color};


#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x: x,
            y: y,
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub position: Position,
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            position: Position::new(LENGTH / 2 , LENGTH / 2),
            direction: Direction::Up,
        }
    }
}

struct Fruit {
    position: Position,
}

impl Fruit {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        Self {
            position: Position::new(rng.random_range(0..LENGTH), rng.random_range(0..LENGTH)),
        }

    }
}
pub struct Game {
    snake: Snake,
    fruit: Fruit,
    alive: bool,
    window: Window,
    buffer: Vec<u32>
}


impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(),
            fruit: Fruit::new(),
            alive: true,
            window: Window::new("Snake Game", LENGTH, LENGTH, WindowOptions::default()).expect("Error opening window"),
            buffer: vec![0; LENGTH * LENGTH],
        }
    }

    pub fn is_running(&self) -> bool {
        return self.alive;
    }

    pub fn init_game(&mut self) {
        crate::paint::create_boarder(&mut self.buffer);
    }

    pub fn update(&mut self) {
        self.window.update_with_buffer(&mut self.buffer, LENGTH,LENGTH);
        paint_blob(&mut self.buffer, &mut self.snake.position, Color::White);
        self.move_snake();
    }

    fn move_snake(&mut self) {
        let old_pos = self.snake.position;
        self.recalc_pos();
        let new_pos = self.snake.position;
        paint_blob(&mut self.buffer, &old_pos, Color::Black);
        paint_blob(&mut self.buffer, &new_pos, Color::White);
    }

     fn recalc_pos(&mut self) {
        let cp =  self.snake.position;
        const MOVE_PIXELS: usize = 5;
        match self.snake.direction {
            Direction::Up => { 
                let new_position = Position::new(cp.x, cp.y - MOVE_PIXELS);
                self.snake.position = new_position;
            },
            Direction::Down => { 
                let new_position = Position::new(cp.x, cp.y + MOVE_PIXELS);
                self.snake.position = new_position;
            },
            Direction::Right => { 
                let new_position = Position::new(cp.x + MOVE_PIXELS, cp.y);
                self.snake.position = new_position;
            },
            Direction::Left => { 
                let new_position = Position::new(cp.x - MOVE_PIXELS, cp.y);
                self.snake.position = new_position;
            },
        }
    }

    pub fn collect_inputs(&mut self) {
        self.window.get_keys_pressed(KeyRepeat::No).iter().for_each(|key|
        match key {
            Key::Up => self.snake.direction = Direction::Up,
            Key::Down => self.snake.direction = Direction::Down,
            Key::Left => self.snake.direction = Direction::Left,
            Key::Right => self.snake.direction = Direction::Right,
            _ => (),
            }
        );
    }
}