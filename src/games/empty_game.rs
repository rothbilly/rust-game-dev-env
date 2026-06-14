use macroquad::prelude::*;
use crate::core::game::{GameLoop};
use crate::core::app::AppRegistration; // To register it in the App registry.


#[derive(Default)]

pub struct EmptyGame;


impl GameLoop for EmptyGame {
    fn _ready(&mut self) {

    }

    fn _process_input(&mut self) {

    }

    fn _update(&mut self, _delta: f32) {

    }

    fn _render(&self) {

    }
}

// This automatically puts this App, which implements "GameLoop", into "AppRegistry" for posterior use.
inventory::submit! {
    AppRegistration {
        name: "EmptyGame",
        constructor: || std::rc::Rc::new(std::cell::RefCell::new(EmptyGame::default())),
    }
}