
use macroquad::{models, prelude::*};
use macroquad::ui::{hash, root_ui, widgets, widgets::Window };

use rust_game_dev_env::editor::{inspector::EventInspector, ui};




fn app_conf() -> Conf{
    Conf {
        window_title: "Dev Environment".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        ..Default::default()
    }  
}


#[macroquad::main(app_conf)]
async fn main() {



    loop{
        clear_background(LIGHTGRAY);

        setup_3d_space();
        //setup_2d_space();
        setup_ui_space();
        
    
        if is_key_down(KeyCode::Escape) || is_key_down(KeyCode::Escape) {
            break;
        }



        
        next_frame().await;
    }
}


fn setup_3d_space(){
        // Going 3d!

        set_camera(&Camera3D {
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            projection: Projection::Perspective,
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), DARKBLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), YELLOW);
        draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);

        
        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);


}


fn setup_2d_space(){
    draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

    //draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);
}


fn setup_ui_space()
{
    EventInspector::draw();

}