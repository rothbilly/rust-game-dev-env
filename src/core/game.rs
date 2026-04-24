use macroquad::prelude::*;

use crate::core::app::App;


pub struct Game{
    base_app: App,
    Score: u32
}

impl Game {
    /// .Create a new game instance
    fn create() -> Game {
        Self {
            App::default(),
            0
        }
    }
}
