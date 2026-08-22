use minifb::{Key, KeyRepeat};
use minifb::{Key::P, Window, WindowOptions};
use crate::WINDOW_SIZE;
use crate::CELL_SIZE;

#[derive(Clone, Copy)]
pub struct Coordinate {
    row: usize,
    column: usize,
}
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
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

    pub fn render(&mut self, row: usize, column: usize, game_object: GameObject) {
        let y_start: usize = row * CELL_SIZE;
        let x_start: usize = column * CELL_SIZE;

        for x_point in x_start..x_start + CELL_SIZE {
            for y_point in y_start..y_start + CELL_SIZE {
                self.paint(x_point, y_point, game_object);
            }
        }
    }

    pub fn paint(&mut self, x_point: usize, y_point: usize, game_object: GameObject) {
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
        // check snakehead == apple | boarder | body
    }

    pub fn move_snake(&mut self) {
        let old_head = self.snake.positions[0];

        let new_head = match self.snake.direction {
            Direction::Up => { Coordinate { column: old_head.column, row: old_head.row - 1 }},
            Direction::Down => { Coordinate { column: old_head.column, row: old_head.row + 1 }},
            Direction::Left => { Coordinate { column: old_head.column - 1, row: old_head.row }},
            Direction::Right => { Coordinate { column: old_head.column + 1, row: old_head.row }},
        };
        let old_tail = self.snake.positions.pop().unwrap();
        self.change_state(old_head, GameObject::SnakeBody);
        self.change_state(new_head, GameObject::SnakeHead);
        self.change_state(old_tail, GameObject::Empty);
        self.snake.positions.insert(0, new_head);
    }

    pub fn change_state(&mut self, position: Coordinate, game_object: GameObject) {
        self.state[position.row][position.column] = game_object;
    }

    pub fn change_direction(&mut self) {
        self.window.get_keys_pressed(KeyRepeat::Yes).iter().for_each(|key|
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

/**
     *  Utility function to encode 0RGB 32 bit pixel buffer
     */
    fn color(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }
