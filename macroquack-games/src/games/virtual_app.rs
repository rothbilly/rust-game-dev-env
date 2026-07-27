
use std::fs;
use macroquack::prelude::*;
use macroquack::core::rendering::vector_graphics::svg_to_srgb_texture;
use macroquad::{color, prelude::*};
use macroquack::core::game::{GameLoop, IPausable};
use macroquack::core::app::AppRegistration;


#[register_app("Sample Virtual App")]
#[derive(Default)]
pub struct SampleVirtualApp{
    _title: String,
    _mode: VirtualAppMode,
    _rect: Rect,
    is_active: bool,
}

const MASCOT_ICON_PATH: &str =  "assets/svgs/quack.svg";



/// # App Mode 
/// Define if the app is used for 2D or 3D Rendering or as editor UI
#[derive(Default)]
pub enum VirtualAppMode {
    #[default]
    Mode2D,
    Mode3D,
    ModeUI,
    Headless
}


impl GameLoop for SampleVirtualApp {
    fn _ready(&mut self){
        set_camera(&Camera2D {
            zoom: vec2(1. / screen_width() * 2., 1. / screen_height() * 2.),
            target: vec2(screen_width() / 2., screen_height() / 2.),
            ..Default::default()
        });
    } 

    fn _render(&self) {
        clear_background(color::LIGHTGRAY);

        if let Ok(svg_str) = fs::read_to_string(MASCOT_ICON_PATH){
            if let Ok(texture) = svg_to_srgb_texture(&svg_str, FilterMode::Nearest){
                draw_texture(&texture, screen_width() * 0.5 - 64. ,screen_height() * 0.5 - 128. , GRAY);
            }
        }

        //draw_text(&format!("FPS: {}", get_fps()), 16., 24., 24., color::RED);
        let intro_text = "MacroQuack Engine";
        let tooltip = "press F3 or Alt to open the inspector";

        let center = get_text_center(intro_text, None, 32, 1.0, 0.);
        let tooltip_center = get_text_center(tooltip, None, 24, 1.0, 0.);
        
        draw_text(intro_text,
        screen_width()/2. - center.x,
        screen_height()/2. - center.y,
        32.,
        color::DARKGRAY);

                draw_text(tooltip,
        screen_width()/2. - tooltip_center.x,
        screen_height()/2. - tooltip_center.y + 32.,
        24.,
        color::GRAY);

    
    }
}




impl IPausable for SampleVirtualApp{
    fn is_active(&self) -> bool{
        self.is_active
    }
    fn set_active(&mut self, _value: bool){
       self.is_active = _value 
    }
}

