use macroquad::prelude::*;
use nanoserde::*;


#[derive(Debug, SerJson, DeJson)]
pub struct Settings{
    volume: f32,
    window_h: u32,
    window_w: u32
};
