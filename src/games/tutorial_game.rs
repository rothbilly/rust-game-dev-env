
use engine_macros::register_app;
use macroquad::experimental::coroutines::*;
use macroquad::prelude::*;
use crate::core::game::{GameLoop};
use crate::core::app::AppRegistration; // To register it in the App registry.
use crate::core::math::wrapf;

const MOVEMENT_SPEED: f32 = 200.0;


#[derive(Default)]
#[register_app("Tutorial Game")]
pub struct TutorialGame{
    dir: Vec2,
    pos: Vec2,
    squares: Vec<Shape>,
    circle: Shape,
    is_gameover: bool
}


impl GameLoop for TutorialGame {
    fn _ready(&mut self) {
        rand::srand(miniquad::date::now() as u64);
        
        self.pos.x = screen_width() / 2.0;
        self.pos.y = screen_height() / 2.0;

        // start_coroutine(async {
        //     wait_seconds(2.).await;
        //     println!("2 seconds passed! Flash")
        // });

    }

    fn _process_input(&mut self) {
        self.dir.x = is_key_down(KeyCode::D) as u8 as f32 -  is_key_down(KeyCode::A) as u8 as f32;
        self.dir.y = is_key_down(KeyCode::S) as u8 as f32 -  is_key_down(KeyCode::W) as u8 as f32;
    }

    fn _update(&mut self, _delta: f32) {



        self.pos += self.dir * MOVEMENT_SPEED * _delta;
        self.pos.x = wrapf(self.pos.x, 0.0, screen_width());
        self.pos.y = wrapf(self.pos.y, 0.0, screen_height());

        
        //squares
        if rand::gen_range(0,99) >= 95{
            let size = rand::gen_range(16.0, 64.0);
            self.squares.push(Shape { size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size
            });
            println!("totals squares: {}", self.squares.len())
        }

        for square in &mut self.squares {
            square.y += square.speed * _delta;
        }
        self.squares.retain(|square| square.y < screen_height() + square.size);
        

    }

    fn _render(&self) {
        clear_background(DARKPURPLE);
        draw_circle(self.pos.x, self.pos.y, 16.0, YELLOW);

        for square in self.squares.iter() {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }

    }
}




#[derive(Default)]
struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}
