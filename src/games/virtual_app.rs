
use macroquad::{color, prelude::*};
use engine_macros::*;
use crate::core::game::{GameLoop, IPausable};
use crate::core::app::AppRegistration;
use crate::core::rendering::vector_graphics::svg_to_srgb_texture;

#[register_app("Sample Virtual App")]
#[derive(Default)]
pub struct SampleVirtualApp{
    _title: String,
    _mode: VirtualAppMode,
    _rect: Rect,
    is_active: bool,
}

const MASCOT_ICON_PATH: &str =  "assets/svgs/quack.svg";
const MASCOT_ICON_STR: &str =  r#"<svg width="128" height="128" viewBox="0 0 128 128" fill="none" xmlns="http://www.w3.org/2000/svg">
<path d="M84.5158 51.4095L73.6843 56.8278L98.9148 93.1116L103.788 79.1972L111.387 90.1531L39.2219 119.607L40.3628 74.6288H67.5506L71.3436 52.1824L82.0957 47.9203L84.5158 51.4095Z" fill="white"/>
<path d="M98.1931 84.7002L79.3718 57.6328L113.685 40.4715L98.1931 84.7002Z" fill="white"/>
<path fill-rule="evenodd" clip-rule="evenodd" d="M73.4253 19.0763L65.169 70.9191L19.3393 72.2522L19.0457 70.1974L56.8178 65.0701L21.4889 45.0105L16.5346 52.5785L10.7202 11.798L73.4253 19.0763ZM52.7274 47.0384L58.5611 45.7488L61.3042 27.0788L53.0838 25.2509L52.7274 47.0384ZM27.2213 21.5554L23.1246 43.0006L31.1271 40.4369L33.0883 20.4261L27.2213 21.5554Z" fill="white"/>
<path d="M47.5654 63.7075L10.6086 68.7246L23.0182 49.77L47.5654 63.7075Z" fill="white"/>
</svg>"#;


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

        if let Ok(texture) = svg_to_srgb_texture(MASCOT_ICON_STR, FilterMode::Nearest){
            draw_texture(&texture, screen_width() * 0.5 - 64. ,screen_height() * 0.5 - 128. , GRAY);
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

