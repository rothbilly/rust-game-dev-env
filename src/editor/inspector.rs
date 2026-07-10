use std::cell::Cell;
use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use egui_macroquad::egui::{self, Color32, Context, Id};
use egui_macroquad::egui::RichText;


use macroquad::ui::{hash, root_ui};
use macroquad::{prelude::*, ui};
use serde::{Deserialize, Serialize};

use crate::data::settings::{GLOB_SETTINGS};

// every inspector on its own mod.
pub mod event_inspector;
pub mod app_loader;
pub mod settings_inspector;

// Define a alias for a common composited type.
type InspectorItem =  Rc<RefCell<dyn InspectorWindow>>;

#[derive(Default, Debug, Serialize,  Deserialize, Clone, Copy)]
pub enum DrawingBackend{
    #[default]
    Macroquad,
    Egui
}


// use cell to allow to set get by auto cloning the value and setting the default in place useful for small values like bool, enum, u32, f32
#[derive(Default)]
pub struct GlobalEditor{
    pub editor_visible: Cell<bool>,
    backend: Cell<DrawingBackend>, 
    editor_windows: HashMap<String, InspectorItem>
}

impl GlobalEditor{


    // create a new inspector and register it.
    pub fn new_inspector<T>(&mut self)
    where T: InspectorWindow + Default + 'static
    {
        let new_inspector = Rc::new(RefCell::new(T::default()));
        let i_name = new_inspector.clone().borrow().get_struct_name();
        self.editor_windows.insert(i_name,  new_inspector);
        self.info_active_editor_names();
    }

    // pass a reference to a currently existing inspector.
    pub fn add_inspector<T>(&mut self, glob_inspector: &Rc<RefCell<T>>)
    where T: InspectorWindow + Default + 'static
    {
        
        self.editor_windows.insert(glob_inspector.borrow().get_struct_name(), glob_inspector.clone()); 
       
    }

    pub fn  toggle_global_visibility(&mut self){
        self.editor_visible.replace(!self.editor_visible.get());
        self.info_active_editor_names();
    }

    pub fn toggle_ui_backend(&self, new_bkd: DrawingBackend ){     
        self.backend.set(new_bkd);
    }

    pub fn info_active_editor_names(&self){
        for (name,_) in self.editor_windows.iter(){
            info!("[Inspector] - {}: is active ", name);
        }
    }

    pub fn _editor_setup(&self){



        for (_, editor) in self.editor_windows.iter(){
            editor.borrow_mut()._setup();
        }
    }

    pub fn _editor_runtime(&self) {
        if self.editor_visible.get()
        {
            match self.backend.get() {

                DrawingBackend::Macroquad => {

                    root_ui().window(hash!(), vec2( 0.0, 0.0),vec2 (screen_width(), 20.0),| ui|{
                        if ui.button(vec2(0.0,0.0), "Exit"){
                            std::process::exit(0);
                        };
                    });


                    for (_, inspector) in self.editor_windows.iter(){
                        inspector.borrow_mut()._draw_gui();
                    }
                }


                DrawingBackend::Egui => {
                    egui_macroquad::ui(|egui_ctx| {
                        //render settings
                        egui_ctx.set_pixels_per_point(GLOB_SETTINGS.read().unwrap().egui_render_scale);
                        egui_ctx.set_theme(egui::Theme::Light);

                        // ==========================================
                        //  Top Bar Panel
                        // ==========================================
                        egui::TopBottomPanel::top(Id::new("MacroQuackTop")).show(egui_ctx, |ui|{
                            egui::menu::bar(ui, |ui|{
                                ui.menu_button("App", |ui|{
                                    if ui.button("Exit").clicked() {
                                        std::process::exit(0);
                                        //ui.close_menu();
                                    }
                                });

                                ui.menu_button("Windows", |ui|{
                                    for (key, editor) in self.editor_windows.iter(){
                                        if ui.button(key).clicked() {
                                            editor.borrow_mut().set_visible(true);
                                        }
                                    }
                                });
                            });
                        });

                        // ==========================================
                        //  Bottom Panel
                        // ==========================================

                        egui::TopBottomPanel::bottom(Id::new("MacroQuackBottom")).show(egui_ctx, |ui|{
                            egui::menu::bar(ui, |ui|{
                                ui.label(RichText::new(&format!("frame time: {:.3} ms", get_frame_time()* 1000.0)).color(Color32::RED));
                                ui.separator();
                                ui.label(&format!("FPS: {}", get_fps()));
                                ui.separator();
                                ui.label(&format!("Game Time: {:.2}", get_time()));
                                
                            });
                        });

                        //egui::SidePanel::right(Id::new("Inspector")).show(egui_ctx, |ui|{});

                        // ==========================================
                        //  Each Inspector Window
                        // ==========================================
                        
                        for (_,inspector) in self.editor_windows.iter(){
                            inspector.borrow_mut()._draw_egui(&egui_ctx);
                        }
                    });
                    egui_macroquad::draw();
                }
            }
        }
    }
}

pub trait InspectorWindow: IVisibility + IName + Inspector{}


pub trait IName
{
    fn get_struct_name(&self) -> String {
        stringify!(type) .to_string() //
    }
}

pub trait IVisibility{
    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, _value: bool);
}

// Draw the GUI
pub trait Inspector {

    fn _setup(&mut self){
    }
    fn _draw_gui(&mut self) ->bool{
        false
    }
    fn _draw_egui(&mut self, _egui_ctx: &Context){

    }
}
