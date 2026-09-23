use std::collections::HashMap;

use macroquack::prelude::*;
use macroquack::core::game::{Game, GameLoop};
use macroquack::core::app::AppRegistration; 

use std::collections::LinkedList;


type Point = (i16, i16);

const SQUARE_SIZE: f32 = 16.0;
const SQUARES: u16 = 16;

const UP: Point = (0, -1);
const DOWN: Point = (0, 1);
const LEFT: Point = (-1, 0); 
const RIGHT: Point = (1, 0); 


#[register_app] 
#[auto_deref(base_game)]
#[derive(Default)]
pub struct SnakeGame{
    base_game: Game,
    snake: Snake,
    fruit: Point,
    score: u32,
    speed: f32,
    last_update: f64,
    is_navigation_lock: bool,
    is_game_over: bool
}


impl GameLoop for SnakeGame {
    fn _ready(&mut self) {
        self.speed = 0.3;
        self.last_update = get_time();
        self.fruit = (rand::gen_range(0, SQUARES as i16),rand::gen_range(0,SQUARES as i16));
        self.is_game_over = false;
        self.is_navigation_lock = false;
    }

    fn _process_input(&mut self) {
        if !self.is_game_over{
            if is_key_down(KeyCode::D) && self.snake.dir != LEFT && !self.is_navigation_lock {
                self.snake.dir = RIGHT;
                self.is_navigation_lock = true;
            } else if is_key_down(KeyCode::A) && self.snake.dir != RIGHT && !self.is_navigation_lock {
                self.snake.dir = LEFT;
                self.is_navigation_lock = true;
            } else if is_key_down(KeyCode::W) && self.snake.dir != DOWN  && !self.is_navigation_lock {
                self.snake.dir = UP;
                self.is_navigation_lock = true;
            } else if is_key_down(KeyCode::S) && self.snake.dir != UP  && !self.is_navigation_lock {
                self.snake.dir = DOWN;
                self.is_navigation_lock = true;
            }
        }
    }

    fn _update(&mut self, _delta: f32) {
        if !self.is_game_over {
                if get_time() - self.last_update > self.speed as f64 {
                    self.last_update = get_time();
                    self.snake.body.push_front(self.snake.head);
                    self.snake.head = (self.snake.head.0 + self.snake.dir.0, self.snake.head.1 + self.snake.dir.1);
                    if self.snake.head == self.fruit {
                        self.fruit = (rand::gen_range(0, SQUARES as i16), rand::gen_range(0, SQUARES as i16));
                        self.score += 100;
                        self.speed *= 0.9;
                    } else {
                        self.snake.body.pop_back();
                    }
                    if self.snake.head.0 < 0
                        || self.snake.head.1 < 0
                        || self.snake.head.0 >= SQUARES as i16
                        || self.snake.head.1 >= SQUARES as i16
                    {
                        self.is_game_over = true;
                    }

                    for (x, y) in &self.snake.body {
                        if *x == self.snake.head.0 && *y == self.snake.head.1 {
                            self.is_game_over = true;
                        }
                    }
                    self.is_navigation_lock = false;
                } 
        } 
    }

    fn _render(&self) {
        if !self.is_game_over {
            clear_background(LIGHTGRAY);

            let game_size = screen_width().min(screen_height());
            let offset_x = (screen_width() - game_size) / 2. + 10.;
            let offset_y = (screen_height() - game_size) / 2. + 10.;
            let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

            draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

            for i in 1..SQUARES {
                draw_line(
                    offset_x,
                    offset_y + sq_size * i as f32,
                    screen_width() - offset_x,
                    offset_y + sq_size * i as f32,
                    2.,
                    LIGHTGRAY,
                );
            }
        }
    }
}

struct Snake {
    head: Point,
    body: LinkedList<Point>,
    dir: Point,
}

impl Default for Snake{
    fn default() -> Self {
        Self { 
            head: (0,0),
            body: LinkedList::new(),
            dir: (1,0) }
    }
}

struct Map<T>{
    tile_size: u32,
    chucks: HashMap<IVec2, Chunk<T>>
}

struct Chunk<T: Sized>{
    tiles: [T; 256]
}