
use macroquack::core::app::AppRegistration;
use macroquack::prelude::*;

use macroquack::core::engine::{Engine, app_config};


#[macroquad::main(app_config)]
async fn main() {

    
    let mut engine = Engine::new();
    engine.setup()
    .with_starting_app()
    //.with_no_editor();
    ;

    loop { // Main loop
        engine.engine_loop();
        next_frame().await;
    }
}


