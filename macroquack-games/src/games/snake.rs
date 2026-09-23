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
        if self.is_game_over{

        } else{

        }
    }

    fn _render(&self) {
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