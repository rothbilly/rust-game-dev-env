use std::path::Path;

use macroquad::prelude::*;
use nanoserde::*;


const PATH_STR: &str = ".app/settings.json";


#[derive(Debug, SerJson, DeJson)]
pub struct Settings{
    volume: f32,
    window_h: u32,
    window_w: u32
};
