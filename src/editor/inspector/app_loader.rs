

use std::rc::Rc;

use macroquad::{math::vec2, ui::{Layout, hash, root_ui, widgets::{Group, Window}}};
use macroquad::prelude::*;
use crate::{core::signal::*, editor::{constants::*, inspector::*}};


use crate::core::app::AppRegistry;

const UI_BLOCK_SIZE: f32 = 40.;


// <'a> defines that main struct cant outlives main_app, so if for some reason main_app is drooped, AppLoader Menu is also drooped.
pub struct AppLoaderMenu{
    is_visible: bool,
    panel_height: f32,
    pub app_registry : Rc<AppRegistry>,
    pub on_click_signal: SignalDispatcher,
}

impl AppLoaderMenu {
    pub fn new(is_visible: bool ) -> Self {
        Self { is_visible, ..Default::default()}
    }


    pub fn try_load_game(&mut self, _option: i32){
        self.on_click_signal.emit(&Signal::IntCallback(_option));
    }
}


impl Default for AppLoaderMenu {
    fn default() -> Self {
        Self { 
            is_visible: true,
            panel_height: UI_BLOCK_SIZE,
            app_registry : Rc::new(AppRegistry::new()),
            on_click_signal: SignalDispatcher::new(),
        }
    }
}


impl Inspector for AppLoaderMenu {


    fn _setup(&mut self){
        self.panel_height = UI_BLOCK_SIZE * self.app_registry.registry.keys().len() as f32;
        // println!("[App Loader Setup] - panel size set to {}", &self.panel_height)
    }

    fn _draw_gui(&mut self) -> bool{

        let window_status = Window::new(hash!(),
            vec2(screen_width() - PANEL_WIDTH - GLOB_MARGIN, GLOB_MARGIN),
            vec2(PANEL_WIDTH, self.panel_height +  PANEL_MARGIN * 2. ))
        .label("App Loader")
        .movable(true)
        .close_button(true)
        .titlebar(true)
        .ui(&mut root_ui(), |ui|{
            Group::new(hash!(), Vec2::new(PANEL_WIDTH - 2.*PANEL_MARGIN,self.panel_height))
                .layout(Layout::Vertical)
                .ui(ui,|ui|{
                    ui.label(None, "App List"); // Not optimal,use a enum in the future

                    for tag in self.app_registry.registry.keys(){
                        if ui.button(None, *tag){
                            self.on_click_signal.emit(&Signal::LoadApp(tag.to_string()));
                        }
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
