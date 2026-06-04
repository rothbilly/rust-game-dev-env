

use macroquad::{math::{self, vec2}, miniquad::log::Level::Warn, ui::{Layout, hash, root_ui, widgets::{Group, Window}}};
use macroquad::prelude::*;
use crate::{core::{app::App, game::GameLoop}, editor::{constants::*, inspector::*}, games::{MacrowareExe, TestGame, empty_game::EmptyGame}};



enum AppList
{
    Empty(EmptyGame),
    Breakout(TestGame),
    Macroware(MacrowareExe)
}



pub struct AppLoaderMenu{
    is_visible: bool,
    position: Vec2,
    main_app: Option<&'static mut App>
}

impl AppLoaderMenu {
    pub fn new(is_visible: bool, position: Vec2, main_app: Option<&'static mut App>) -> Self {
        Self { is_visible, position, main_app }
    }


    pub fn load_game(&mut self, selected: AppList){
        if let Some(app_ref) = &mut self.main_app {
            // app_ref: &mut &'static mut App -> deref to get &mut App
            let app: &mut App = &mut **app_ref;
            match selected {
                AppList::Empty(game) => {
                    app.add_sub_app(Box::new(game));
                }
                AppList::Breakout(game) => {
                    app.add_sub_app(Box::new(game));
                }
                AppList::Macroware(game) => {
                    app.add_sub_app(Box::new(game));
                }
                
            }
            println!("the app status is {app:?}");

        } else {
            warn!("not main app assigned to this")
        }
    }
}

impl AppLoaderMenu{
    //App should be static for the program lifetime and outlive this inspector 
    fn setup_app(&mut self, _app: &'static mut App ) -> &mut Self{
        self.main_app = Some(_app);
        self
    }
}

impl Default for AppLoaderMenu {
    fn default() -> Self {
        Self { 
            is_visible: true, 
            position: Vec2::ZERO,
            main_app: None
        }
    }
}


impl Inspector for AppLoaderMenu {


    fn _draw_gui(&mut self) -> bool{
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
                        self.load_game(AppList::Empty(EmptyGame));
                    }

                    if ui.button( None,"Breakout Game"){
                        self.load_game(AppList::Macroware(MacrowareExe::default()));
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

impl InspectorWindow for AppLoaderMenu{}
