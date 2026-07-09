use crate::core::app::AppRegistration; // To register it in the App registry.
use crate::core::game::GameLoop;
use engine_macros::register_app;
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;

pub const EMPTY_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.0);
pub const SOFTBLUE_COLOR: Color = Color::new(0.66, 0.71, 0.89, 1.0);
pub const SAND_COLOR: Color = Color::new(0.89, 0.84, 0.66, 1.0);
pub const GUN_POWDER_COLOR: Color = Color::new(0.122, 0.125, 0.165, 1.0);

/*
    IMPORTANT: improve single thread performance before adding multithreading
    TODO: add spatial partitioning and dirty rects
    TODO: add UI back
    maybe add brush patterns
*/

// Screen references
// Snes NTSC 256 * 224 @60Hz PAL 256 * 240 @50Hz
// GBA 240 * 160 3:2 @60Hz
// maybe change this to be "Game size/res??"
const WORLD_WIDTH: i32 = 256 * 2;
const WORLD_HEIGHT: i32 = 240 * 2;
const ASPECT_RATIO: f32 = WORLD_WIDTH as f32 / WORLD_HEIGHT as f32;

#[register_app("Sandbox")] // Custom Registration macro
pub struct Sandbox {
    selected_tile_type: u8,
    available_tiles: [u8; 10],
    brush_size: i16,
    max_brush_size: i16,
    tiles: Vec<u8>,
    next_tiles: Vec<u8>,
    frame_counter: u64,
    world_image: Image,
    world_texture: Texture2D,
    camera: Camera2D,
    draw_debug: bool,
}

impl Default for Sandbox {
    fn default() -> Self {
        Self {
            selected_tile_type: SAND,
            available_tiles: [
                LAVA, SAND, WATER, FIRE, GUN_POWDER, WOOD, STEEL, VAPOR, SMOKE, SNOW,
            ],
            brush_size: 4,
            max_brush_size: 64,
            tiles: Vec::<u8>::new(),
            next_tiles: Vec::<u8>::new(),
            frame_counter: 0,
            world_image: Image::empty(),
            world_texture: Texture2D::empty(),
            camera: Camera2D::default(),
            draw_debug: true,
            base: Default::default(),
        }
    }
}

impl GameLoop for Sandbox {
    fn _ready(&mut self) {
        let vec_size = (WORLD_WIDTH * WORLD_HEIGHT) as usize;
        self.tiles = vec![SAND; vec_size];
        self.next_tiles = vec![SAND; vec_size];
        self.world_image = Image::gen_image_color(WORLD_WIDTH as u16, WORLD_HEIGHT as u16, BLACK);
        self.world_texture = Texture2D::from_image(&self.world_image);
        self.world_texture.set_filter(FilterMode::Nearest);
    }

    fn _process_input(&mut self) {
        let pressed_keys = get_keys_pressed();
        for key in &pressed_keys {
            let key_value = *key as i32;
            let idx = key_value - KeyCode::Key0 as i32;
            if idx >= 0 && idx <= 9 {
                self.selected_tile_type = self.available_tiles[idx as usize];
            }
        }

        let mouse_wheel = mouse_wheel();

        self.brush_size = self.brush_size + (mouse_wheel.1.round() * 2.0) as i16;
        self.brush_size = self.brush_size.clamp(1, self.max_brush_size);

        let mouse = self.mouse_to_world();

        let grid_x = mouse.x.floor() as i16;
        let grid_y = mouse.y.floor() as i16;

        let mut tile_type = self.selected_tile_type;
        let mouse_left = is_mouse_button_down(MouseButton::Left);
        let mouse_right = is_mouse_button_down(MouseButton::Right);
        if !mouse_left && !mouse_right {
            return;
        }

        if mouse_right {
            tile_type = EMPTY;
        }

        let brush_size = self.brush_size;

        for y in 0..brush_size {
            for x in 0..brush_size {
                let tile_x = grid_x - (brush_size as i16 / 2) + x as i16;
                let tile_y = grid_y - (brush_size as i16 / 2) + y as i16;

                if !self.in_bounds((tile_x, tile_y)) {
                    continue;
                }

                let idx = self.pos_to_idx((tile_x, tile_y));

                self.tiles[idx] = tile_type;
            }
        }
    }

    fn _update(&mut self, _delta: f32) {
        if self.frame_counter % 2 == 0 {
            for y in 0..WORLD_HEIGHT {
                for x in (0..WORLD_WIDTH).rev() {
                    self.check_tile(&(x as i16), &(y as i16));
                }
            }
        } else {
            for y in 0..WORLD_HEIGHT {
                for x in 0..WORLD_WIDTH {
                    self.check_tile(&(x as i16), &(y as i16));
                }
            }
        }

        std::mem::swap(&mut self.next_tiles, &mut self.tiles);
        self.next_tiles.fill_with(|| EMPTY);

        for i in 0..self.tiles.len() {
            let tile = self.tiles[i];
            self.world_image.set_pixel(
                (i % WORLD_WIDTH as usize) as u32,
                (i / WORLD_WIDTH as usize) as u32,
                get_tile_color(&tile),
            );
        }
        self.world_texture.update(&self.world_image);
        self.frame_counter = (self.frame_counter + 1) % 60;
        self.update_camera();
    }

    fn _render(&self) {
        set_camera(&self.camera);
        draw_texture(&self.world_texture, 0.0, 0.0, WHITE);

        let mouse = self.mouse_to_world();
        let brush_size = self.brush_size as f32;
        draw_rectangle_lines(
            mouse.x - (brush_size / 2.0),
            mouse.y - (brush_size / 2.0),
            brush_size,
            brush_size,
            4.0,
            WHITE,
        );
    }
}

impl Sandbox {
    fn update_camera(&mut self) {
        let mut camera = Camera2D::from_display_rect(Rect {
            x: 0.0,
            y: WORLD_HEIGHT as f32,
            w: WORLD_WIDTH as f32,
            h: -WORLD_HEIGHT as f32,
        });
        let display = self.get_display_rect();
        camera.viewport = Some((
            display.x as i32,
            display.y as i32,
            display.w as i32,
            display.h as i32,
        ));

        self.camera = camera;
    }

    fn get_display_rect(&self) -> Rect {
        let screen_size = screen_size();
        let dest_size: (f32, f32);
        let x: f32;
        let y: f32;
        if screen_size.0 > screen_size.1 {
            dest_size = (screen_size.1 * ASPECT_RATIO, screen_size.1);
            x = screen_size.0 / 2.0 - dest_size.0 / 2.0;
            y = 0.0;
        } else {
            dest_size = (screen_size.0, screen_size.0 / ASPECT_RATIO);
            x = 0.0;
            y = screen_size.1 / 2.0 - dest_size.1 / 2.0;
        }
        Rect {
            x,
            y,
            w: dest_size.0,
            h: dest_size.1,
        }
    }

    #[inline]
    fn mouse_to_world(&self) -> Vec2 {
        let mouse = mouse_position();
        self.camera.screen_to_world(vec2(mouse.0, mouse.1))
    }

    #[inline]
    fn check_tile(&mut self, x: &i16, y: &i16) {
        let idx = self.pos_to_idx((*x, *y));
        let tile = self.tiles[idx];
        if tile == EMPTY {
            return;
        }
        if !self.process_tile(tile, (*x, *y)) {
            self.next_tiles[idx] = tile;
        }
    }

    fn process_tile(&mut self, tile: u8, pos: (i16, i16)) -> bool {
        let neighbors = self.get_neighbors_per_type(pos);

        // we take advantage of the way conditions are validated, the || (or) operator will not check any other option if the first one is true
        match tile {
            SAND => self.try_move(pos, (pos.0, pos.1 + 1)) || self.try_move_down_sides(pos),
            GUN_POWDER => {
                if neighbors[FIRE as usize] > 0 && self.roll(0.6) {
                    if self.roll(0.6) {
                        self.set_tile(self.pos_to_idx(pos), FIRE);
                    } else {
                        self.set_tile(self.pos_to_idx(pos), SMOKE);
                    }
                    return true;
                }

                self.try_move(pos, (pos.0, pos.1 + 1)) || self.try_move_down_sides(pos)
            }
            WATER => {
                if neighbors[FIRE as usize] >= 4 {
                    self.set_tile(self.pos_to_idx(pos), VAPOR);
                    return true;
                } else if neighbors[EMPTY as usize] >= 3 && self.roll(0.0002) {
                    self.set_tile(self.pos_to_idx(pos), VAPOR);
                }

                (self.roll(0.05) && self.try_move_sides(pos))
                    || self.try_move(pos, (pos.0, pos.1 + 1))
                    || self.try_move_down_sides(pos)
                    || self.try_move_sides(pos)
            }

            SMOKE => {
                if neighbors[FIRE as usize] >= 2 && self.roll(0.6) {
                    self.set_tile(self.pos_to_idx(pos), FIRE);

                    return true;
                }

                if self.roll(0.0003) {
                    self.set_tile(self.pos_to_idx(pos), EMPTY);
                }

                (self.roll(0.25) && self.try_move_up_sides(pos))
                    || self.try_move(pos, (pos.0, pos.1 - 1))
                    || self.try_move_sides(pos)
                    || self.try_move(pos, (pos.0, pos.1 + 1))
            }
            VAPOR => {
                if neighbors[WATER as usize] >= 4 {
                    self.set_tile(self.pos_to_idx(pos), WATER);
                    return true;
                } else if neighbors[VAPOR as usize] >= 3 && self.roll(0.000001) {
                    self.set_tile(self.pos_to_idx(pos), WATER);
                    return true;
                }

                if self.roll(0.0003) {
                    self.set_tile(self.pos_to_idx(pos), EMPTY);
                }

                (self.roll(0.25) && self.try_move_up_sides(pos))
                    || self.try_move(pos, (pos.0, pos.1 - 1))
                    || self.try_move_sides(pos)
                    || self.try_move(pos, (pos.0, pos.1 + 1))
            }
            SNOW => {
                if neighbors[FIRE as usize] >= 1 {
                    self.set_tile(self.pos_to_idx(pos), WATER);

                    return true;
                } else if neighbors[WATER as usize] >= 3 && self.roll(0.01) {
                    self.set_tile(self.pos_to_idx(pos), WATER);
                    return true;
                } else if neighbors[VAPOR as usize] >= 2 && self.roll(0.3) {
                    self.set_tile(self.pos_to_idx(pos), WATER);
                    return true;
                }

                if self.roll(0.10) {
                    let rng = rand::gen_range(0, 100);

                    return match rng {
                        0..50 => self.try_move_down_sides(pos),
                        50..100 => self.try_move(pos, (pos.0, pos.1 + 1)),
                        _ => false,
                    };
                }
                false
            }

            FIRE => {
                if neighbors[WATER as usize] >= 4 {
                    self.set_tile(self.pos_to_idx(pos), VAPOR);
                    return true;
                } else if neighbors[GUN_POWDER as usize] > 1 {
                    self.set_tile(self.pos_to_idx(pos), SMOKE);
                    return true;
                }

                if self.roll(0.02) {
                    self.set_tile(self.pos_to_idx(pos), EMPTY);
                    return true;
                }

                let rng = rand::gen_range(0, 100);

                match rng {
                    0..20 => self.try_move(pos, (pos.0, pos.1 + 1)),
                    20..40 => self.try_move_down_sides(pos),
                    40..60 => self.try_move_sides(pos),
                    60..80 => self.try_move_up_sides(pos),
                    _ => self.try_move(pos, (pos.0, pos.1 - 1)),
                }
            }
            WOOD => {
                if neighbors[FIRE as usize] > neighbors[WATER as usize] && self.roll(0.2) {
                    if self.roll(0.7) {
                        self.set_tile(self.pos_to_idx(pos), BURNTWOOD);
                    } else {
                        self.set_tile(self.pos_to_idx(pos), SMOKE);
                    }
                    return true;
                }
                false
            }
            BURNTWOOD => {
                if neighbors[FIRE as usize] - neighbors[WATER as usize] > 1 {
                    let rng = rand::gen_range(0, 100);
                    match rng {
                        0..20 => self.set_tile(self.pos_to_idx(pos), FIRE),
                        20..40 => self.set_tile(self.pos_to_idx(pos), SMOKE),
                        _ => self.set_tile(self.pos_to_idx(pos), COAL),
                    }
                    return true;
                }
                self.try_move(pos, (pos.0, pos.1 + 1))
                    || (self.roll(0.05) && self.try_move_down_sides(pos))
            }
            COAL => {
                if (neighbors[FIRE as usize] - neighbors[WATER as usize]) > 0 && self.roll(0.05) {
                    if self.roll(0.4) {
                        self.set_tile(self.pos_to_idx(pos), FIRE);
                    } else {
                        self.set_tile(self.pos_to_idx(pos), SMOKE);
                    }
                    return true;
                }
                self.try_move(pos, (pos.0, pos.1 + 1))
                    || (self.roll(0.05) && self.try_move_down_sides(pos))
            }
            _ => false,
        }
    }

    fn try_move(&mut self, from: (i16, i16), to: (i16, i16)) -> bool {
        let can = self.is_empty(to);
        if can {
            self.move_tile(from, to);
        }
        can
    }

    #[inline]
    fn try_move_down_sides(&mut self, from: (i16, i16)) -> bool {
        let down_right = self.roll(0.5);

        if down_right {
            self.is_empty((from.0 + 1, from.1)) && self.try_move(from, (from.0 + 1, from.1 + 1))
        } else {
            self.is_empty((from.0 - 1, from.1)) && self.try_move(from, (from.0 - 1, from.1 + 1))
        }
    }

    #[inline]
    fn try_move_up_sides(&mut self, from: (i16, i16)) -> bool {
        let up_right = self.roll(0.5);

        if up_right {
            self.is_empty((from.0 + 1, from.1)) && self.try_move(from, (from.0 + 1, from.1 - 1))
        } else {
            self.is_empty((from.0 - 1, from.1)) && self.try_move(from, (from.0 - 1, from.1 - 1))
        }
    }

    #[inline]
    fn try_move_sides(&mut self, from: (i16, i16)) -> bool {
        let right = self.roll(0.5);

        if right {
            self.try_move(from, (from.0 + 1, from.1))
        } else {
            self.try_move(from, (from.0 - 1, from.1))
        }
    }

    fn move_tile(&mut self, from: (i16, i16), to: (i16, i16)) {
        let from_idx = self.pos_to_idx(from);
        let to_idx = self.pos_to_idx(to);
        let tile = self.tiles[from_idx];
        self.next_tiles[to_idx] = tile;
        self.tiles[from_idx] = EMPTY;
    }

    // the index is the type
    fn get_neighbors_per_type(&self, from: (i16, i16)) -> [i8; TILE_TYPES] {
        let positions = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let mut neighbors = [0 as i8; TILE_TYPES];

        let mut i: usize = 0;
        let mut tile: u8;
        for pos in &positions {
            let n_pos = (from.0 + pos.0, from.1 + pos.1);

            if !self.in_bounds(n_pos) {
                continue;
            }

            tile = self.tiles[self.pos_to_idx(n_pos)];

            neighbors[tile as usize] = neighbors[tile as usize] + 1;

            i = i + 1;
        }

        neighbors
    }

    #[inline]
    fn set_tile(&mut self, idx: usize, tile_type: u8) {
        self.next_tiles[idx] = tile_type;
    }

    #[inline]
    fn is_empty(&self, pos: (i16, i16)) -> bool {
        let idx = self.pos_to_idx(pos);
        self.in_bounds(pos) && self.tiles[idx] == EMPTY && self.next_tiles[idx] == EMPTY
    }

    #[inline]
    fn pos_to_idx(&self, pos: (i16, i16)) -> usize {
        (pos.1 as i32 * WORLD_WIDTH + pos.0 as i32) as usize
    }

    #[inline]
    fn in_bounds(&self, pos: (i16, i16)) -> bool {
        (pos.0 >= 0)
            && (pos.0 < WORLD_WIDTH as i16)
            && (pos.1 >= 0)
            && (pos.1 < WORLD_HEIGHT as i16)
    }

    #[inline]
    fn roll(&self, chance: f32) -> bool {
        assert!(
            chance <= 1.0 && chance >= 0.0,
            "Chance for a roll must be equals or less than 1.0 and more than 0.0"
        );
        return rand::gen_range(0.0, 1.0) < chance;
    }
}

const EMPTY: u8 = 0;
const STEEL: u8 = 1;
const SAND: u8 = 2;
const WATER: u8 = 3;
const SMOKE: u8 = 4;
const VAPOR: u8 = 5;
const SNOW: u8 = 6;
const WOOD: u8 = 7;
const BURNTWOOD: u8 = 8;
const COAL: u8 = 9;
const FIRE: u8 = 10;
const GUN_POWDER: u8 = 11;
const LAVA: u8 = 12;
const TILE_TYPES: usize = 13;

pub fn get_tile_color(val: &u8) -> Color {
    match *val {
        EMPTY => EMPTY_COLOR,
        STEEL => SOFTBLUE_COLOR,
        SAND => SAND_COLOR,
        WATER => SKYBLUE,
        SMOKE => GRAY,
        VAPOR => LIGHTGRAY,
        SNOW => WHITE,
        WOOD => BROWN,
        BURNTWOOD => DARKBROWN,
        COAL => DARKGRAY,
        FIRE => ORANGE,
        GUN_POWDER => GUN_POWDER_COLOR,
        LAVA => RED,
        _ => MAGENTA,
    }
}
