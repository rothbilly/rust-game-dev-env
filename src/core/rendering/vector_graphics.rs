use macroquad::{texture::{FilterMode, Texture2D}};


use resvg::{tiny_skia, usvg::{self, Size, Transform}};
use serde::de::Unexpected::Option;


const svg_sample_data: &str = r##"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="400" height="400">
            <circle cx="50" cy="50" r="40" fill="#ff5555" />
            <rect x="30" y="30" width="40" height="40" fill="#55ff55" opacity="0.7"/>
        </svg>
    "##;


#[derive(Debug)]
pub enum SVGParseError {
    SVGSourceParse,
    NoFonts,
    InvalidTransform,
    PNGEncoding,
}


pub fn svg_to_srgb_texture(svg_str: &str, filter_mode: FilterMode) -> Result<Texture2D, SVGParseError>{
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg_str, &opt)
        .expect("Failed to parse SVG");    

    let size = tree.size();
    let width = size.width() as u32;
    let height = size.height() as u32;

    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .expect("Failed to create pixmap");


    resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());


    let svg_texture  = Texture2D::from_rgba8(
        width as u16,
        height as u16,
        pixmap.data());

    svg_texture.set_filter(filter_mode);

    Ok(svg_texture)
}