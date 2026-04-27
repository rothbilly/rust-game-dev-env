use macroquad::prelude::*;

use crate::core::game::{Game, GameLoop};


const BLOCKS_W: usize = 10;
const BLOCKS_H: usize = 10;
const SCR_W: f32 = 20.0;
const SCR_H: f32 = 20.0;


macro_rules! fields_mut {
    ($self:ident, $($field:ident),* $(,)?) => {
        $(let $field = &mut $self.$field;)*
    };
}


#[derive(Default)]
pub struct TestGame {
    base_game: Game,
    blocks: [[bool; BLOCKS_W]; BLOCKS_H],
    ball_x: f32,
    ball_y: f32,
    dx: f32,
    dy:f32,
    platform_x: f32,
    stick: bool,
    platform_width: f32,
    platform_height: f32,

}


impl GameLoop for TestGame {
    fn _ready(&mut self) -> &mut Self {
        self.blocks = [[true; BLOCKS_W]; BLOCKS_H];
        self.ball_x = 12.;
        self.ball_y = 7.;
        self.dx = 3.5;
        self.dy = -3.5;
        self.platform_x = 10.;
        self.stick = true;
        self.platform_width = 5.;
        self.platform_height = 0.2;


        set_camera(&Camera2D {
            zoom: vec2(1. / SCR_W * 2., 1. / SCR_H * 2.),
            target: vec2(SCR_W / 2., SCR_H / 2.),
            ..Default::default()
        });


        println!("game ready");
        return self
    }
    
    fn _update(&mut self, _delta: f32) {

        println!("game update {}", get_time());
        if is_key_down(KeyCode::Right) && self.platform_x < SCR_W - self.platform_width / 2. {
            self.platform_x += 3.0 * _delta;
        }
        if is_key_down(KeyCode::Left) && self.platform_x > self.platform_width / 2. {
            self.platform_x -= 3.0 * _delta;
        }

        if self.stick == false {
            self.ball_x += self.dx * _delta;
            self.ball_y += self.dy * _delta;
        } else {
            let (font_size, font_scale, font_aspect) = camera_font_scale(1.);
            let text_params = TextParams {
                font_size,
                font_scale,
                font_scale_aspect: font_aspect,
                ..Default::default()
            };
            draw_text_ex(
                "Press space to start",
                SCR_W / 2. - 5.,
                SCR_H / 2.,
                text_params,
            );

            self.ball_x = self.platform_x;
            self.ball_y = SCR_H - 0.5;

            self.stick = !is_key_down(KeyCode::Space);

        }
                if self.ball_x <= 0. || self.ball_x > SCR_W {
            dx *= -1.;
        }
        if self.ball_x <= 0.
            || (self.ball_x > SCR_H - platform_height - 0.15 / 2.
                && self.ball_x >= platform_x - platform_width / 2.
                && self.ball_x <= platform_x + platform_width / 2.)
        {
            dy *= -1.;
        }
        if self.ball_x >= SCR_H {
            self.ball_x = 10.;
            dy = -dy.abs();
            stick = true;
        }

        for j in 0..BLOCKS_H {
            for i in 0..BLOCKS_W {
                if blocks[j][i] {
                    let block_w = SCR_W / BLOCKS_W as f32;
                    let block_h = 7.0 / BLOCKS_H as f32;
                    let block_x = i as f32 * block_w + 0.05;
                    let block_y = j as f32 * block_h + 0.05;

                    draw_rectangle(block_x, block_y, block_w - 0.1, block_h - 0.1, DARKBLUE);
                    if self.ball_x >= block_x
                        && self.ball_x < block_x + block_w
                        && self.ball_x >= block_y
                        && self.ball_x < block_y + block_h
                    {
                        dy *= -1.;
                        blocks[j][i] = false;
                    }
                }
            }
        }

        draw_circle(self.ball_x, self.ball_x, 0.2, RED);
        draw_rectangle(
            platform_x - platform_width / 2.,
            SCR_H - platform_height,
            platform_width,
            platform_height,
            DARKPURPLE,
        );

        
    }
}



