

use std::ops::Deref;


use macroquad::{math::{self, vec2}, miniquad::log::Level::Warn, ui::{Layout, hash, root_ui, widgets::{Group, Window}}};
use macroquad::prelude::*;
use crate::{core::{app::App, game::GameLoop, signal::*}, editor::{constants::*, inspector::*}, games::{MacrowareExe, TestGame, empty_game::EmptyGame}};


const UI_BLOCK_SIZE: f32 = 40.;


// <'a> defines that main struct cant outlives main_app, so if for some reason main_app is drooped, AppLoader Menu is also drooped.

pub struct AppLoaderMenu{
    is_visible: bool,
    position: Vec2,
    pub on_click_signal: SignalDispatcher
}

impl AppLoaderMenu {
    pub fn new(is_visible: bool, position: Vec2, ) -> Self {
        Self { is_visible, position, on_click_signal: SignalDispatcher::new()}
    }


    pub fn try_load_game(&mut self, _option: i32){
        self.on_click_signal.emit(&Signal::IntCallback(_option));
    }
}


impl Default for AppLoaderMenu {
    fn default() -> Self {
        Self { 
            is_visible: true, 
            position: Vec2::ZERO,
            on_click_signal: SignalDispatcher::new()
        }
    }
}


impl Inspector for AppLoaderMenu {


    fn _setup(&mut self){

    }

    fn _draw_gui(&mut self) -> bool{
        let window_status = Window::new(hash!(),
            vec2(screen_width() - PANEL_WIDTH - GLOB_MARGIN, GLOB_MARGIN),
            vec2(PANEL_WIDTH, UI_BLOCK_SIZE * 3.))
        .label("App Loader")
        .movable(true)
        .close_button(true)
        .titlebar(true)
        .ui(&mut root_ui(), |ui|{
            Group::new(hash!(), Vec2::new(PANEL_WIDTH - 2.*PANEL_MARGIN,90.))
                .layout(Layout::Vertical)
                .ui(ui,|ui|{
                    ui.label(None, "App List"); // Not optimal,use a enum in the future

                    if ui.button( None,"Start Screen"){
                        self.try_load_game(0);
                    }

                    if ui.button( None,"MacroExe"){
                        self.try_load_game(1);
                    }

                    if ui.button( None,"Breakout"){
                        self.try_load_game(2); 
                    }
                });
    });
    window_status
    }
}


impl IVisibility for AppLoaderMenu{
    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn set_visible(&mut self, _value: bool) {
        self.is_visible = _value
    }
}

impl InspectorWindow for AppLoaderMenu {}
