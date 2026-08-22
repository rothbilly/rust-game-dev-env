use crate::{sandbox::*, tile::Tile};

use macroquack::prelude::*;

#[derive(Debug)]
pub struct PaintBrush {
    pub _drawing: bool,
    pub tile: Tile,
    pub size: i16,
    pub max_size: i16,
    pub sampler_img: Image,
    pub sampler_tex: Texture2D,
    pub sampler: RasterPaintBrushPattern,
    pub pattern: RasterPaintBrushPattern,
}

impl Default for PaintBrush {
    fn default() -> Self {
        let max_size: i16 = 64;
        let tmp = vec![0; (max_size * max_size) as usize];

        PaintBrush {
            tile: Tile::Sand,
            _drawing: false,
            size: 1,
            max_size,
            sampler_img: Image::gen_image_color((max_size / 4) as u16, max_size as u16, BLACK),
            sampler_tex: Texture2D::from_rgba8((max_size / 4) as u16, max_size as u16, &tmp),
            sampler: RasterPaintBrushPattern::empty(),
            pattern: RasterPaintBrushPattern::circle(),
        }
    }
}

impl PaintBrush {
    pub fn set_size(&mut self, size: i16) {
        self.size = size;
        self.resize_sampler();
    }

    pub fn set_tile(&mut self, tile: Tile) {
        if self.tile != tile {
            self.tile = tile;
            self.refresh_sampler();
        }
    }

    pub fn paint_line_segment(
        &mut self,
        p1: IVec2,
        p2: IVec2,
        paint_fn: &mut dyn FnMut(IVec2, Tile),
    ) {
        if p1 == p2 {
            self.paint(p1, paint_fn);
        }

        let dx = (p2.x - p1.x).abs();
        let sx = if p1.x < p2.x { 1 } else { -1 };
        let dy = -((p2.y - p1.y).abs());
        let sy = if p1.y < p2.y { 1 } else { -1 };
        let mut err = dx + dy;

        let mut p = p1;
        loop {
            self.paint(p, paint_fn);
            let err2 = 2 * err;
            if err2 >= dy {
                if p.x == p2.x {
                    break;
                }
                err += dy;
                p.x += sx;
            }
            if err2 <= dx {
                if p.y == p2.y {
                    break;
                }
                err += dx;
                p.y += sy;
            }
        }
    }

    pub fn paint(&mut self, p: IVec2, paint_fn: &mut dyn FnMut(IVec2, Tile)) {
        let size = self.size;

        if size != self.sampler.size {
            self.resize_sampler();
        }

        for y in 0..size {
            let py = y - size / 2;
            for x in 0..size {
                let px = x - size / 2;
                let p = p + ivec2(px as i32, py as i32);

                if !in_world_bounds(p) {
                    continue;
                }

                if self.sampler.data[(y * size + x) as usize] == 0 {
                    continue;
                }
                paint_fn(p, self.tile);
            }
        }
    }

    pub fn refresh_sampler(&mut self) {
        if self.size != self.sampler.size {
            self.resize_sampler();
        }

        for byte in &mut self.sampler_img.bytes {
            if *byte != 0 {
                *byte = self.tile as u8;
            }
        }
    }

    pub fn resize_sampler(&mut self) {
        let new_size = self.size;

        let img_size = self.max_size as i32;
        let img_center = img_size / 2;

        for byte in &mut self.sampler_img.bytes {
            *byte = 0;
        }

        //if new_size == 1 {
        //    self.sampler = RasterPaintBrushPattern::square();
        //    self.sampler_img.bytes[(img_center / 2 * img_size + img_center) as usize] =
        //        self.tile as u8;
        //    return;
        //}

        self.sampler.size = new_size;
        self.sampler.data.resize((new_size * new_size) as usize, 0);

        for y in 0..new_size {
            for x in 0..new_size {
                let img_y = img_center - (y - ((new_size - 1) / 2)) as i32;
                let sample_y: usize =
                    ((y as f32 / new_size as f32) * self.pattern.size as f32).floor() as usize;
                let sample_x: usize =
                    ((x as f32 / new_size as f32) * self.pattern.size as f32).floor() as usize;

                let img_x = img_center - (x - ((new_size - 1) / 2)) as i32;
                if self.pattern.data[sample_y * self.pattern.size as usize + sample_x] > 0 {
                    self.sampler.data[(y * new_size + x) as usize] = 1_u8;
                    self.sampler_img.bytes[(img_y * img_size + img_x) as usize] = self.tile as u8;
                } else {
                    self.sampler.data[(y * new_size + x) as usize] = 0_u8;
                    self.sampler_img.bytes[(img_y * img_size + img_x) as usize] = 0_u8;
                };
            }
        }

        self.sampler_tex.update(&self.sampler_img);
    }
}

#[derive(Debug)]
pub struct RasterPaintBrushPattern {
    size: i16,
    data: Vec<u8>,
}

impl RasterPaintBrushPattern {
    pub fn empty() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    pub fn circle() -> Self {
        let size: i16 = 64;
        let mut data = vec![0; (size * size) as usize];

        let r = size as f32 / 2.0;

        for y in 0..size {
            let py = y as f32 - (size as f32 / 2.0) + 0.5;
            for x in 0..size {
                let px = x as f32 - (size as f32 / 2.0) + 0.5;
                if (vec2(px, py).length() - r) <= 0.0 {
                    data[(y * size + x) as usize] = 1;
                }
            }
        }

        Self { size, data }
    }

    pub fn square() -> Self {
        Self {
            size: 1,
            data: vec![1; 1],
        }
    }
}
