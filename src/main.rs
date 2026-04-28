use std::collections::{HashSet, HashMap};
use std::hash::Hash;

use macroquad::{models, prelude::*};
use macroquad::ui::{hash, root_ui, widgets, widgets::Window };

use crate::core::game::GameLoop;
use crate::editor::constants::*;
use crate::editor::{inspector::EventInspector, ui};
use crate::games::test_game::TestGame;



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
    
    const WORLD_UP: Vec3 = vec3(0.0,1.0,0.0);

    let mut camera_pos: Vec3 = vec3(-20., 15., 0.);
    let mut camera_target: Vec3 = vec3(0., 0., 0.);

    let mut input_01: String = "sample".to_owned();
    let mut test_game: Box<TestGame> = Box::from(TestGame::default());
    test_game._ready();

    loop{
        let delta = get_frame_time();


        clear_background(LIGHTGRAY);
        
        //passing camera_pos as mutable reference &mut Vec3 type ()
        //update_3d_camera_pos(&mut camera_pos, &camera_target); 
        //draw_3d_space();
        //set_default_camera();
        
        test_game._update(delta);
        //draw_2d_space();
        //draw_ui_space(&mut input_01);
        
        
        if is_key_down(KeyCode::Escape) {
            break;
        }
        
        
        next_frame().await;
    }
}


const CAM_VELOCITY: f32 = 0.2;



fn update_3d_camera_pos(_camera_pos: &mut Vec3, _camera_target: &Vec3){

        let facing_dir:Vec3 = *_camera_target - *_camera_pos;


        let mut camera_dir = Vec3::ZERO; 


        if is_key_down(KeyCode::A){
            camera_dir.z += 1.0;
        } else if is_key_down(KeyCode::D) {
            camera_dir.z -= 1.0
        }

        if is_key_down(KeyCode::W){
            camera_dir.x += 1.0;
        } else if is_key_down(KeyCode::S) {
            camera_dir.x -= 1.0
        }



     
        *_camera_pos =  * _camera_pos + camera_dir * CAM_VELOCITY;


        set_camera(&Camera3D {
            position:  *_camera_pos as Vec3,
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            projection: Projection::Perspective,
            ..Default::default()
        });

}


fn draw_3d_space(){
    // Going 3d!

        draw_grid(20, 1., BLACK, GRAY);

        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), DARKBLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), YELLOW);
        draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);
    


}


fn draw_2d_space(){
    draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

    //draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);
}


fn draw_ui_space(_input: &mut String )
{
    EventInspector::draw();
    

    Window::new(hash!(), vec2(1280. - PANEL_WIDTH - GLOB_MARGIN,20.), vec2(PANEL_WIDTH, 100.))
        .label("Inspector")
        .movable(true)
        .close_button(true)
        .titlebar(true)
        .ui(&mut *root_ui(), |ui|{

            ui.editbox(
                hash!(),
                vec2(PANEL_WIDTH - PANEL_MARGIN,80.),
                _input);
        });

}


#[derive(PartialEq, Eq, Clone)]
struct InspectableData<T>
where T: Hash + PartialEq + Eq + Clone,
{
    data: HashMap<String, T>
}

impl<T> InspectableData<T>
where T: Hash + PartialEq + Eq + Clone,
{
    
    fn new() -> Self{
        Self{ data: HashMap::new()}
    }

    fn new_with(data: HashMap<String, T>) -> Self {
        Self { data }
    }
    
    pub fn add_value(&mut self, _key: String, _value: T){

    }
}