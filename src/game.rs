use minifb::{Key, KeyRepeat};
use minifb::{Window, WindowOptions};
use rand::{RngExt};
use crate::{MATRIX_SIZE, WINDOW_SIZE};
use crate::CELL_SIZE;

#[derive(Clone, Copy)]
pub struct Coordinate {
    row: usize,
    column: usize,
}
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameObject {
    Boarder,
    Empty,
    SnakeHead,
    SnakeBody,
    Apple
}

pub struct Game {
    alive: bool,
    window: Window,
    state: Vec<Vec<GameObject>>,
    buffer: Vec<u32>,
    snake: Snake,
    score: u8,
}

pub struct Snake {
    positions: Vec<Coordinate>,
    direction: Direction,
}

impl Snake {
    pub fn new(positions: Vec<Coordinate>) -> Self {
        Self {
            positions,
            direction: Direction::Right,
        }
    }
}

impl Game {
    pub fn new() -> Self {
        use GameObject:: {
            Boarder as B,
            Empty as E,
            SnakeHead as H,
            SnakeBody as S,
            Apple as A,
        };
        // 10 x 10 matrix
        let state = vec![
            vec![B, B, B, B, B, B, B, B, B, B],
            vec![B, E, E, E, E, E, E, E, E, B],
            vec![B, E, E, E, E, A, E, E, E, B],
            vec![B, E, E, E, E, E, E, E, E, B],
            vec![B, E, E, E, E, E, E, E, E, B],
            vec![B, E, S, S, H, E, E, E, E, B],
            vec![B, E, E, E, E, E, E, E, E, B],
            vec![B, E, E, E, E, E, E, E, E, B],
            vec![B, E, E, E, E, E, E, E, E, B],
            vec![B, B, B, B, B, B, B, B, B, B],
        ];

        let snake_positions = vec![
            Coordinate {column: 4, row: 5},
            Coordinate {column: 3, row: 5},
            Coordinate {column: 2, row: 5},
        ];

        Self {
            alive: true,
            window: Window::new("Snake Game", WINDOW_SIZE, WINDOW_SIZE, WindowOptions::default()).expect("Error opening window"),
            state,
            buffer: vec![0; WINDOW_SIZE * WINDOW_SIZE],
            snake: Snake::new(snake_positions),
            score: 0,
        }
    }

    pub fn is_running(&self) -> bool {
        return self.alive;
    }

    pub fn render_game(&mut self) {
        for row in 0..self.state.len() {
            for column in 0..self.state[row].len() {
                let element = self.state[row][column];
                self.render(row, column, element);
            }
        }
        self.window.update_with_buffer(&mut self.buffer, WINDOW_SIZE, WINDOW_SIZE).expect("test");
    }

    fn render(&mut self, row: usize, column: usize, game_object: GameObject) {
        let y_start: usize = row * CELL_SIZE;
        let x_start: usize = column * CELL_SIZE;

        for x_point in x_start..x_start + CELL_SIZE {
            for y_point in y_start..y_start + CELL_SIZE {
                self.paint(x_point, y_point, game_object);
            }
        }
    }

    fn paint(&mut self, x_point: usize, y_point: usize, game_object: GameObject) {
        let color = match game_object {
            GameObject::Boarder => color(0, 0, 139),
            GameObject::Empty => color(0, 0, 0),
            GameObject::Apple => color(255, 0, 0),
            GameObject::SnakeBody => color(255, 255, 255),
            GameObject::SnakeHead => color(0, 255, 0),
        };
        self.buffer[y_point * WINDOW_SIZE + x_point] = color;
    }

    pub fn update(&mut self) {

        self.move_snake();
    }

    fn is_dead_or_apple(&mut self, head: &Coordinate) -> bool {
        match self.state[head.row][head.column] {
            GameObject::Boarder => {self.alive = false; false},
            GameObject::SnakeBody => {self.alive = false; false},
            GameObject::Apple => {self.apple_eaten(); true},
            _ => {false},
        }
    }

    pub fn move_snake(&mut self) {
        // define important coordinates
        let old_head = self.snake.positions[0];
        let new_head = match self.snake.direction {
            Direction::Up => { Coordinate { column: old_head.column, row: old_head.row - 1 }},
            Direction::Down => { Coordinate { column: old_head.column, row: old_head.row + 1 }},
            Direction::Left => { Coordinate { column: old_head.column - 1, row: old_head.row }},
            Direction::Right => { Coordinate { column: old_head.column + 1, row: old_head.row }},
        };

        let apple: bool = self.is_dead_or_apple(&new_head);
        if !apple {
            let old_tail = self.snake.positions.pop().unwrap();
            self.change_state(old_tail, GameObject::Empty);
        }

        // change state matrix
        self.change_state(old_head, GameObject::SnakeBody);
        self.change_state(new_head, GameObject::SnakeHead);
        
        // add new head to snake vector
        self.snake.positions.insert(0, new_head);
    }

    fn change_state(&mut self, position: Coordinate, game_object: GameObject) {
        self.state[position.row][position.column] = game_object;
    }

    pub fn change_direction(&mut self) {
        self.window.get_keys_pressed(KeyRepeat::No).iter().for_each(|key|
        match key {
            Key::Up => {
                if self.snake.direction != Direction::Down || self.snake.direction == Direction::Up {
                    self.snake.direction = Direction::Up;
                }
            }
            Key::Down => {
                if self.snake.direction != Direction::Up || self.snake.direction == Direction::Down {
                    self.snake.direction = Direction::Down;
                }
            }
            Key::Left => {
                if self.snake.direction != Direction::Right || self.snake.direction == Direction::Left {
                    self.snake.direction = Direction::Left;
                }
            }
            Key::Right => {
                if self.snake.direction != Direction::Left || self.snake.direction == Direction::Right {
                    self.snake.direction = Direction::Right;
                }
            }
            _ => (),
        }
    );
    }

    fn apple_eaten(&mut self) {
        self.score += 1;
        let mut rng = rand::rng();

        loop {
            let row = rng.random_range(0..MATRIX_SIZE - 1);
            let column = rng.random_range(0..MATRIX_SIZE - 1);
            let object = self.state[row][column];

            if object == GameObject::Empty {
                self.state[row][column] = GameObject::Apple;
                return;
            }
        }
    }

    pub fn get_score(&self) -> u8 {
        self.score
    }
    
}

/**
     *  Utility function to encode 0RGB 32 bit pixel buffer
     */
    fn color(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }