
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::RwLock; 


use macroquad::math::Vec2;
use macroquad:: prelude::*;
// egui
use egui_macroquad::*;


use rust_game_dev_env::core::rendering::post_processing::{CameraMode, ScreenVisualEffect};
use rust_game_dev_env::editor::inspector::settings_inspector::SettingsInspector;
use rust_game_dev_env::editor::inspector::{DrawingBackend, GlobalEditor};

// own crate imports
use rust_game_dev_env as game_lib; 
use game_lib::core::signal::Signal;
use game_lib::editor::inspector::app_loader::AppLoaderMenu;
use game_lib::core::{app::*};
use game_lib::data::settings::{Settings, GLOB_SETTINGS};
use game_lib::editor::inspector::event_inspector::*;
use game_lib::games::virtual_app::SampleVirtualApp;



// This struct is from Macroquad and it's used to set parameters of the window.
fn app_conf() -> Conf {
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

#[macroquad::main(app_conf)]
async fn main() {

    // If Non Static - Reference Counted Rc, Interior Mutability by a RefCell/RwLock -- Maybe in the future use Arc<RwLock>
    let main_app: Rc<RwLock<App>> = Rc::new(RwLock::new(App::initialize_and_ready::<SampleVirtualApp>())); 
    let global_inspectors: Rc<RwLock<GlobalEditor>> = Rc::new(RwLock::new(GlobalEditor::default()));

    let app_loader = Rc::new(RefCell::new(AppLoaderMenu::default()));

    global_inspectors.write().unwrap().new_inspector::<EventInspector>();
    global_inspectors.write().unwrap().add_inspector::<AppLoaderMenu>(&app_loader);
    global_inspectors.write().unwrap().new_inspector::<SettingsInspector>();

    global_inspectors.write().unwrap().toggle_ui_backend(DrawingBackend::Egui);

    let app_registry = app_loader.borrow().app_registry.clone();
    
    
    let closure_app = main_app.clone(); // Rc count 2 -- app_clone is moved inside the closure because of `move`
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
    
    let loop_app_ref = main_app.clone(); // Rc count 3

    global_inspectors.read().unwrap()._editor_setup();
    let crt = ScreenVisualEffect::create_from_sample(Vec2{x: screen_width()/2.0, y: screen_height()/2.0}, FilterMode::Nearest);

    loop { // Main loop
        let mut delta = get_frame_time();
        delta = delta.max(0.0).min(0.1);

        if is_key_pressed(KeyCode::F3) || is_key_pressed(KeyCode::LeftAlt){
            global_inspectors.write().unwrap().toggle_global_visibility();
            loop_app_ref.write().unwrap().toggle_debug_view();
        }


        // Game Layer
        loop_app_ref.read().unwrap()._runtime(delta);
        // VFX Layer
        //crt.render_vfx();
        // Editor Layer
        global_inspectors.read().unwrap()._editor_runtime(); // the order of drawing is important 

        if is_key_down(KeyCode::Escape) {
            on_close_application();
            std::process::exit(0);
        }

        next_frame().await;
    }
}

fn on_close_application() {
    let _ = GLOB_SETTINGS.read()
        .unwrap()
        .save();
}
