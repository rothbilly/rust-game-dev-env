
use macroquad::{math::{self, vec2}, ui::{Layout, hash, root_ui, widgets::{Group, Window}}};
use macroquad::prelude::*;
use crate::{core::{app::App, game::GameLoop}, editor::{constants::*, inspector::*}, games::{MacrowareExe, TestGame, empty_game::EmptyGame}};



enum AppList
{
    Empty(EmptyGame),
    Breakout(TestGame),
    MacrowareExe(MacrowareExe)
}

pub struct AppLoaderMenu{
    is_visible: bool,
    main_app: &'static mut App
}

impl AppLoaderMenu {

    pub fn load_a(&mut self, selected: AppList){
        match selected {
            AppList::Empty(game) => {
                self.main_app.add_sub_app(Box::new(game));
            }
            AppList::Breakout(game) => {
                self.main_app.add_sub_app(Box::new(game));
            }
            AppList::MacrowareExe(game) => {
                self.main_app.add_sub_app(Box::new(game));
            }
        }
    }
}




impl Inspector for AppLoaderMenu {
    fn _draw_gui(&self) -> bool{
        let window_status = Window::new(hash!(), vec2(GLOB_MARGIN, GLOB_MARGIN), vec2(PANEL_WIDTH, PANEL_HEIGHT))
        .label("App Loader")
        .movable(true)
        .close_button(true)
        .titlebar(true)
        .ui(&mut root_ui(), |ui|{
            Group::new(hash!(), Vec2::new(PANEL_WIDTH - 2.*PANEL_MARGIN,90.))
                .layout(Layout::Vertical)
                .ui(ui,|ui|{
                    ui.label(None, "App List");

                    if ui.button( None,"Test Game"){

                    }
                });

            Group::new(hash!(), Vec2::new(PANEL_WIDTH - 2.*PANEL_MARGIN,90.))
            .layout(Layout::Vertical)
            .ui(ui, |ui|{
                ui.label(None, "Pressed mouse keys");
                
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

impl InspectorWindow for AppLoaderMenu{}
