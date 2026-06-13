

use std::rc::Rc;

use egui_macroquad::egui::mutex::RwLock;
use macroquad::{math::{self, vec2}, ui::{Layout, hash, root_ui, widgets::{Group, Window}}};
use macroquad::prelude::*;
use crate::{core::{app::App, signal::*}, editor::{constants::*, inspector::*}, games::{MacrowareExe, TestGame, empty_game::EmptyGame}};

use std::convert::TryFrom;

const UI_BLOCK_SIZE: f32 = 40.;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppType{

    SampleVirtualApp,
    MacroExe,
    Breakout,
}

impl AppType{
    pub fn label(&self) -> &'static str {
        match self {
            AppType::SampleVirtualApp => "SampleVirtualApp",
            AppType::MacroExe => "MacrowareExe",
            AppType::Breakout => "Breakout",
        }
    }
}

impl TryFrom<i32> for AppType{
    type Error = ();

    fn try_from(v : i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(AppType::SampleVirtualApp),
            1 => Ok(AppType::MacroExe),
            2 => Ok(AppType::Breakout),
            _ => Err(()),
        }
    }
}


// <'a> defines that main struct cant outlives main_app, so if for some reason main_app is drooped, AppLoader Menu is also drooped.
pub struct AppLoaderMenu{
    is_visible: bool,
    position: Vec2,
    available_apps : Vec<AppType>,
    pub on_click_signal: SignalDispatcher,
    pub main_app_ref: Option<Rc<RwLock<App>>>
}

impl AppLoaderMenu {
    pub fn new(is_visible: bool, position: Vec2, ) -> Self {
        Self { is_visible, position, ..Default::default()}
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
            available_apps: vec![AppType::SampleVirtualApp, AppType::MacroExe, AppType::Breakout],
            on_click_signal: SignalDispatcher::new(),
            main_app_ref: None
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

                    for app in &self.available_apps.clone(){
                        if ui.button(None, app.label()){
                            self.try_load_game(*app as i32)
                        }
                    }
                    // if ui.button( None,"Start Screen"){
                    //     self.try_load_game(0);
                    // }

                    // if ui.button( None,"MacroExe"){
                    //     self.try_load_game(1);
                    // }

                    // if ui.button( None,"Breakout"){
                    //     self.try_load_game(2); 
                    // }
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
