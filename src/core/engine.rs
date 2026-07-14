use std::{cell::RefCell, ops::Deref, rc::Rc, sync::RwLock};

use macroquad::prelude::*;

use crate::{core::{app::{AppList, AppRegistry}, signal::Signal}, data::settings::{GLOB_SETTINGS, Settings}, editor::inspector::{DrawingBackend, GlobalEditor, app_loader::AppLoaderMenu, event_inspector::EventInspector, settings_inspector::SettingsInspector}, games::virtual_app::SampleVirtualApp};


type AppListRc = Rc<RwLock<AppList>>;
type AppRegistryRc = Rc<RwLock<AppRegistry>>;
type EditorsRc = Rc<RwLock<GlobalEditor>>;

pub struct Engine{
    app_list: AppListRc,
    app_registry:AppRegistryRc,
    global_inspectors: EditorsRc
}

impl Engine {
    pub fn new() -> Engine{
        Engine{

            app_list: Rc::new(RwLock::new(AppList::default())),
            app_registry: Rc::new(RwLock::new(AppRegistry::new())),
            global_inspectors: Rc::new(RwLock::new(GlobalEditor::default()))
        }
    }

    pub fn setup(&mut self) -> &mut Self{

    let app_loader = Rc::new(RefCell::new(AppLoaderMenu::default()));

    self.global_inspectors.write().unwrap().add_inspector::<AppLoaderMenu>(&app_loader);
    self.global_inspectors.write().unwrap().new_inspector::<EventInspector>();
    self.global_inspectors.write().unwrap().new_inspector::<SettingsInspector>();
    self.global_inspectors.write().unwrap().toggle_ui_backend(DrawingBackend::Egui);

    let app_registry = app_loader.borrow().app_registry.clone();

    
    let closure_app = self.app_list.clone(); // Rc count 2 -- app_clone is moved inside the closure because of `move`
    // TODO!: move this to the editor or the app, and use dependency injection in the future to cleanup the main.
    app_loader.borrow_mut().on_click_signal.connect(move |signal| {
        if let Signal::LoadApp(tag) = signal {
            if let Some(new_app) = app_registry.get(tag){
                let mut app_write = closure_app.write().unwrap();

                app_write.load_and_replace_app_at_last(new_app);
                //app_write.get_total_apps();
            }else{
                println!("[Editor] Error: Tag {} not in registry.", tag);
            }
        }
    });

    self.global_inspectors.read().unwrap()._editor_setup();
    //let crt = ScreenVisualEffect::create_from_sample(Vec2{x: screen_width()/2.0, y: screen_height()/2.0}, FilterMode::Nearest);

    self
    }

    pub fn engine_loop(&self){
        let mut delta = get_frame_time();
        delta = delta.max(0.0).min(0.1);

        if is_key_pressed(KeyCode::F3) || is_key_pressed(KeyCode::LeftAlt){
            self.global_inspectors.write().unwrap().toggle_global_visibility();
            self.app_list.write().unwrap().toggle_debug_view();
        }


        // Game Layer
        self.app_list.read().unwrap()._runtime(delta);
        // VFX Layer
        //crt.render_vfx();
        // Editor Layer
        self.global_inspectors.read().unwrap()._editor_runtime(); // the order of drawing is important 

        if is_key_down(KeyCode::Escape) {
            self.on_close_application();
        }
    }

    pub fn on_close_application(&self) {
        let _ = GLOB_SETTINGS.read()
            .unwrap()
            .save();

         std::process::exit(0);
    }

    pub fn with_starting_app(&mut self) -> &mut Self{
        let rc_binding = self.app_registry.clone();
        let registry = rc_binding.read().unwrap(); 
        let settings = GLOB_SETTINGS.read().unwrap();
        if let Some(new_app) = registry.get(&settings.starting_app){
            self.app_list.clone().write().unwrap().append_new_sub_app(new_app);
        } else {
            self.app_list.clone().write().unwrap().new_app_as::<SampleVirtualApp>();   
        }
        self
    }

    pub fn with_no_editor(&mut self) -> &mut Self{
        self
    }



}


// This struct is from Macroquad and it's used to set parameters of the window.
pub fn app_config() -> Conf {
    let mut settings_guard: std::sync::RwLockWriteGuard<'_, Settings> = GLOB_SETTINGS.write().unwrap();
    let settings = settings_guard.load_toml();
    
    Conf {
        window_title: "MacroQuack".to_owned(),
        window_width: settings.window_w,
        window_height: settings.window_h,
        window_resizable: true,
        fullscreen: settings.fullscreen,
        platform: {
            miniquad::conf::Platform {
                swap_interval: Some(settings.swap_interval),
                ..Default::default()
            }
        },
        ..Default::default()
    }
}