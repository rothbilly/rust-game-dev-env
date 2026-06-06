use std::sync::{LazyLock, Mutex}; // For async.
use macroquad:: prelude::*;
use egui_macroquad::*;


use rust_game_dev_env::editor::inspector::Inspector;
use rust_game_dev_env::editor::inspector::app_loader::AppLoaderMenu;
//crate imports
use rust_game_dev_env as game_lib; 
use game_lib::core::{app};
use game_lib::data::{settings::Settings};
use game_lib::editor::inspector::event_inspector::*;
use game_lib::core::app::virtual_app::SampleVirtualApp;



/// static = eternal lifetime
/// LazyLock = delays execution until it is called on code. Avoids having to define static at compile time.
/// Mutex = Can only be edited under a lock. Avoids race conditions.
static GLOB_SETTINGS: LazyLock<Mutex<Settings>> = LazyLock::new(|| Mutex::new(Settings::new()));

// This struct is from Macroquad and it's used to set parameters of the window.
fn app_conf() -> Conf {
    let settings = GLOB_SETTINGS.lock().unwrap().load_toml();

    Conf {
        window_title: "Dev Environment".to_owned(),
        window_width: settings.window_w,
        window_height: settings.window_h,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(app_conf)]
async fn main() {
    let mut app = app::App::initialize_and_ready::<SampleVirtualApp>();

    let mut ev = EventInspector::default();
    let mut editor_visible: bool = false;

    loop { // App loop
        let mut delta = get_frame_time();
        delta = delta.max(0.0).min(0.1);


        if is_key_pressed(KeyCode::F3) || is_key_pressed(KeyCode::LeftAlt){
            editor_visible = !editor_visible;
        }
        
        if editor_visible{
            ev._draw_gui();
        }

        app._runtime(delta);

        if is_key_down(KeyCode::Escape) {
            on_close_application();
            break;

        }
        next_frame().await;
    }
}

fn on_close_application() {
    println!("[Application] - Closing!");
    let _ = GLOB_SETTINGS        .lock()
        .unwrap()
        .save();

}

