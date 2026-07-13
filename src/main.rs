
use macroquad:: prelude::*;
// egui
use egui_macroquad::*;

use rust_game_dev_env::core::engine::Engine;
use rust_game_dev_env::data::settings::{Settings, GLOB_SETTINGS};


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

    let mut engine = Engine::new();
    engine.setup();


    loop { // Main loop
        engine.engine_loop();
        next_frame().await;
    }
}


