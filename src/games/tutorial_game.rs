
use engine_macros::register_app;
use std::fs;

use macroquad::prelude::*;
use crate::core::game::{GameLoop};
use crate::core::app::AppRegistration; // To register it in the App registry.
use crate::core::math::wrapf;

const MOVEMENT_SPEED: f32 = 200.0;
const SCORE_PATH: &str = ".app/games/tutorial_game/high_score.dat";


#[register_app("Tutorial Game")]
#[derive(Default)]
pub struct TutorialGame{
    dir: Vec2,
    squares: Vec<Shape>,
    bullets: Vec<Shape>,
    circle: Shape,
    is_game_over: bool,
    score: u32,
    high_score: u32
}


impl GameLoop for TutorialGame {
    fn _ready(&mut self) {
        rand::srand(miniquad::date::now() as u64);
        
        self.circle.x = screen_width() / 2.0;
        self.circle.y = screen_height() / 2.0;
        self.circle.size = 16.0;
        self.circle.speed = MOVEMENT_SPEED.clone();
        // start_coroutine(async {
        //     wait_seconds(2.).await;
        //     println!("2 seconds passed! Flash")
        // });
        self.score = 0;
        self.high_score = fs::read_to_string(&SCORE_PATH)
            .map_or(Ok(0),|i| {i.parse::<u32>()})
            .unwrap_or(0);


    }

    fn _process_input(&mut self) {
        self.dir.x = is_key_down(KeyCode::D) as u8 as f32 -  is_key_down(KeyCode::A) as u8 as f32;
        self.dir.y = is_key_down(KeyCode::S) as u8 as f32 -  is_key_down(KeyCode::W) as u8 as f32;

    }

    fn _update(&mut self, _delta: f32) {

        if self.is_game_over{

            if self.score >= self.high_score {

                fs::write(SCORE_PATH, self.high_score.to_string()).ok();
            }
            self.score = 0;


            if is_key_pressed(KeyCode::Space){ //reset
                self.squares.clear();
                self.bullets.clear();
                self.circle.x = screen_height() * 0.5;
                self.circle.y = screen_width() * 0.5; // using glam vec2
                self.is_game_over = false
            }
        
        // Main game loop
        } else {

            self.circle.set_pos( self.circle.get_pos() + self.dir * self.circle.speed * _delta);
            self.circle.x = wrapf(self.circle.x, 0.0, screen_width());
            self.circle.y = wrapf(self.circle.y, 0.0, screen_height());
            
            //Spawn Squares
            if rand::gen_range(0,99) >= 95{
                let size = rand::gen_range(16.0, 64.0);
                self.squares.push(Shape { size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    is_collided: false
                });
                // println!("totals squares: {}", self.squares.len())
            }

            //Shoot
            if is_key_pressed(KeyCode::Space){
                self.bullets.push(Shape {
                    x: self.circle.x,
                    y: self.circle.y,
                    speed: self.circle.speed * 2.0,
                    size: 5.0,
                    is_collided: false,
                });
            }

            // Move Squares
            for square in &mut self.squares {
                square.y += square.speed * _delta;
            }
            for bullet in &mut self.bullets {
                bullet.y -= bullet.speed * _delta;
            }

            // Check Collisions
            if  self.squares.iter().any(|square| self.circle.collides_with(square)){
                self.is_game_over = true
            }

            for square in self.squares.iter_mut() {
                for bullet in self.bullets.iter_mut() {
                    if bullet.collides_with(square) {
                        bullet.is_collided = true;
                        square.is_collided = true;

                        self.score += square.size.round() as u32;
                        self.high_score = self.high_score.max(self.score);
                    }
                }
            }

            // Retain or remove from the game
            self.squares.retain(|square| square.y < screen_height() + square.size);
            self.bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);

            self.squares.retain(|square| {!square.is_collided});
            self.bullets.retain(|bullet| {!bullet.is_collided});

        }
    }

    fn _render(&self) {
        clear_background(DARKPURPLE);
        draw_circle(self.circle.x, self.circle.y, self.circle.size, YELLOW);

        if self.is_game_over{
            let game_over_text = "GAME OVER";
            let text_dimensions = measure_text(game_over_text, None, 50, 1.0);
            draw_text(game_over_text, screen_width() * 0.5 - text_dimensions.width * 0.5, screen_height() * 0.5, 50., WHITE);
        }

        for bullet in &self.bullets {
            draw_circle(bullet.x, bullet.y, bullet.size / 2.0, RED);
        }

        for square in self.squares.iter() {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }

        draw_text(
            format!("Score: {}", self.score).as_str(),
            10.0,
            35.0,
            25.0,
            WHITE,
        );
        let high_score_text = format!("High score: {}", self.high_score);

        let text_dimensions = measure_text(high_score_text.as_str(), None, 25, 1.0);
        draw_text(
            high_score_text.as_str(),
            screen_width() - text_dimensions.width - 10.0,
            35.0,
            25.0,
            WHITE,
        );

    }
}




#[derive(Default)]
struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    is_collided:bool
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

    fn set_pos(&mut self, pos: Vec2){
        self.x = pos.x;
        self.y = pos.y;
    }

    fn get_pos(&self) -> Vec2{
        Vec2::new(self.x , self.y)
    }
}
