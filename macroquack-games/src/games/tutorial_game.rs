
use macroquack::prelude::*;
use std::fs;

use macroquad::prelude::*;
use macroquack::core::game::{Game, GameLoop};
use macroquack::core::app::AppRegistration; // To register it in the App registry.
use macroquack::core::math::wrapf;

const MOVEMENT_SPEED: f32 = 200.0;
const SCORE_PATH: &str = ".app/storage/tutorial_game/";
const SCORE_FILE_NAME: &str = "high_score.dat";

 const FRAGMENT_SHADER: &str = include_str!(r"../../../assets/shaders/starfield_shader.glsl");
 const VERTEX_SHADER: &str = include_str!(r"../../../assets/shaders/quad_vertex.glsl");


#[register_app("Tutorial Game")]
#[auto_deref(base_game)]
#[derive(Default)]
pub struct TutorialGame{
    base_game: Game,
    dir: Vec2,
    squares: Vec<Shape>,
    bullets: Vec<Shape>,
    circle: Shape,
    score: u32,
    high_score: u32,
    game_state: GameState,
    bg_material: Option<Material>,
    render_target: Option<RenderTarget>,
    direction_modifier: f32
}


impl TutorialGame{
    pub fn reset(&mut self){
    self.squares.clear();
    self.bullets.clear();
    self.circle.x = screen_width() * 0.5;
    self.circle.y = screen_height() * 0.5;
    self.score = 0;
    self.game_state = GameState::Playing;
    }
}


impl GameLoop for TutorialGame {
    fn _ready(&mut self) {
        rand::srand(miniquad::date::now() as u64);
    
        self.circle.size = 16.0;
        self.circle.speed = MOVEMENT_SPEED;

        // start_coroutine(async {
        //     wait_seconds(2.).await;
        //     println!("2 seconds passed! Flash")
        // });

        if self.bg_material.is_none(){
            self.render_target = Some(render_target(320, 150));

            if let Some(rt)  = &self.render_target {
                rt.texture.set_filter(FilterMode::Nearest);
            }
            self.direction_modifier = 0.0;
            self.bg_material = Some(load_material(ShaderSource::Glsl {
                vertex: VERTEX_SHADER,
                 fragment: FRAGMENT_SHADER 
                },
            MaterialParams {
                uniforms: vec![
                        UniformDesc::new("iResolution", UniformType::Float2),
                        UniformDesc::new("direction_modifier", UniformType::Float1),             
                ],
                ..Default::default()
            }).unwrap());

        };

        //println!("[Sample Game] - Read from in {}{}", SCORE_PATH, SCORE_FILE_NAME);
        self.high_score = fs::read_to_string(format!("{}{}", SCORE_PATH, SCORE_FILE_NAME))
            .map_or(Ok(0),|i| {i.parse::<u32>()})
            .unwrap_or(0);

    }

    fn _process_input(&mut self) {
        
        match self.game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Space) {
                    self.reset();
                }
            }
            GameState::Playing => {
                self.dir.x = is_key_down(KeyCode::D) as u8 as f32 -  is_key_down(KeyCode::A) as u8 as f32;
                self.dir.y = is_key_down(KeyCode::S) as u8 as f32 -  is_key_down(KeyCode::W) as u8 as f32;
        
                

                if is_key_pressed(KeyCode::Backspace) {
                    self.game_state = GameState::Paused;
                }
            },
            GameState::Paused => {

                if is_key_pressed(KeyCode::Backspace) {
                    self.game_state = GameState::Playing;
                }
            },
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space){ 
                    self.game_state = GameState::MainMenu;
                }
            },
        }
    }

    fn _update(&mut self, _delta: f32) {
        //println!("{:#?}", &self.game_state);

        match self.game_state {
            GameState::MainMenu => {}
            GameState::Playing => {

                self.direction_modifier += 0.05 * self.dir.x * _delta;
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
                    self.game_state = GameState::GameOver;
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

            },
            GameState::Paused => {},
            GameState::GameOver => {
                if self.score == self.high_score {

                    println!("[Sample Game] - Write on {}{}", SCORE_PATH, SCORE_FILE_NAME);

                    fs::create_dir_all(SCORE_PATH).ok();
                    fs::write(
                        format!("{}{}", SCORE_PATH, SCORE_FILE_NAME),
                        self.high_score.to_string()).unwrap();
                    self.score = 0;
                }
            }
        }
    }

    fn _render(&self) {
        clear_background(BLACK);

        if let Some(material) = &self.bg_material {
            material.set_uniform("iResolution", (screen_width(), screen_height()));
            material.set_uniform("direction_modifier", self.direction_modifier);

            gl_use_material(material);
            if let Some(rt) = &self.render_target{
                draw_texture_ex(
                    &rt.texture,
                    0.,
                    0.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(screen_width(), screen_height())),
                        ..Default::default()
                    },
                );
                gl_use_default_material();
            }
        }

        match self.game_state {
            GameState::MainMenu => {
                let text = "Press space";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
            }
            GameState::Playing => {
                draw_circle(self.circle.x, self.circle.y, self.circle.size, YELLOW);

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

            },
            GameState::Paused => {
                let text = "Paused";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
            },
            GameState::GameOver => {
                let game_over_text = "GAME OVER";
                let text_dimensions = measure_text(game_over_text, None, 50, 1.0);
                draw_text(game_over_text, screen_width() * 0.5 - text_dimensions.width * 0.5, screen_height() * 0.5, 50., WHITE);
            },
        }

    }


    fn _debug_draw(&self) {
        match self.game_state {
            GameState::Playing => {
                draw_line(
                    self.circle.x,
                    self.circle.y,
                    self.circle.x + self.dir.x * 20.,
                    self.circle.y + self.dir.y * 20.,
                    4.0,
                    RED);
            }
            _ => {}
        }
    }


    fn _cleanup(&mut self) {
        clear_background(WHITE);
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


#[derive(Default, Debug)]
enum GameState{
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver
}