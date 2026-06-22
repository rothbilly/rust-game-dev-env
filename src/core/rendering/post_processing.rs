


use macroquad::prelude::*;
use macroquad::math::Vec2;


pub enum CameraMode {
    Camera2DMode,
    Camera3DMode
}

pub struct ScreenVisualEffect{
    texture_size: Vec2,
    render_target: RenderTarget,
    filter_mode: FilterMode,
    material: Material
}


impl ScreenVisualEffect {

    // Call this BEFORE the main loop
    pub fn create_from_sample(_size: Vec2, filter_mode: FilterMode) -> ScreenVisualEffect{
        
        //Create a new render target from the size
        let render_target = render_target(_size.x as u32, _size.y as u32);
        render_target.texture.set_filter(filter_mode);

        let material = load_material(
            ShaderSource::Glsl { 
                vertex: CRT_VERTEX_SHADER,
                fragment: CRT_FRAGMENT_SHADER 
            },
            Default::default()
        ).unwrap();

        ScreenVisualEffect { 
            texture_size: _size,
            render_target,
            filter_mode,
            material
            }
    }
    // this need to be refactor, each game set its own camera as its need, maybe each game renders to a global target idk.
    // Use this BEFORE render the scene during loop
    pub fn setup_test_renderer_camera(&self,camera_mode: CameraMode){

        match camera_mode {
            CameraMode::Camera2DMode =>{
                set_camera(&Camera2D{ 
                    zoom: vec2(1. / screen_width() * 2., 1. / screen_height() * 2.),
                    target: vec2(screen_width() / 2., screen_height() / 2.),
                    render_target: Some(self.render_target.clone()),
                    ..Default::default()
                });
            }
             CameraMode::Camera3DMode => {
                set_camera(&Camera3D{ 
                    ..Default::default()
                });
             }
        }        
    }

    // Use this AFTER render the scene during loop
    pub fn render_vfx(&self){

        set_default_camera();
        clear_background(WHITE);
        gl_use_material(&self.material);
        
        draw_texture_ex(
            &self.render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();
    }


}



// ==========================================
//  Shaders provided for the example
// ==========================================



const CRT_FRAGMENT_SHADER: &'static str = r#"#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

// https://www.shadertoy.com/view/XtlSD7

vec2 CRTCurveUV(vec2 uv)
{
    uv = uv * 2.0 - 1.0;
    vec2 offset = abs( uv.yx ) / vec2( 6.0, 4.0 );
    uv = uv + uv * offset * offset;
    uv = uv * 0.5 + 0.5;
    return uv;
}

void DrawVignette( inout vec3 color, vec2 uv )
{
    float vignette = uv.x * uv.y * ( 1.0 - uv.x ) * ( 1.0 - uv.y );
    vignette = clamp( pow( 16.0 * vignette, 0.3 ), 0.0, 1.0 );
    color *= vignette;
}


void DrawScanline( inout vec3 color, vec2 uv )
{
    float iTime = 0.1;
    float scanline 	= clamp( 0.95 + 0.05 * cos( 3.14 * ( uv.y + 0.008 * iTime ) * 240.0 * 1.0 ), 0.0, 1.0 );
    float grille 	= 0.85 + 0.15 * clamp( 1.5 * cos( 3.14 * uv.x * 640.0 * 1.0 ), 0.0, 1.0 );
    color *= scanline * grille * 1.2;
}

void main() {
    //vec2 crtUV = CRTCurveUV(uv);
    vec3 res = texture2D(Texture, uv).rgb * color.rgb;
    // if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0)
    // {
    //     res = vec3(0.0, 0.0, 0.0);
    // }
    //DrawVignette(res, crtUV);
    DrawScanline(res, uv);
    gl_FragColor = vec4(res, 1.0);

}
"#;

const CRT_VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}
";