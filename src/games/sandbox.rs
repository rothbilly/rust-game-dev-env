use crate::core::app::AppRegistration; // To register it in the App registry.
use crate::core::game::GameLoop;
use engine_macros::register_app;
use macroquad::miniquad::window::screen_size;

use macroquad::prelude::*;
use quad_rand::ChooseRandom;

pub const EMPTY: Color = Color::new(0.0, 0.0, 0.0, 0.0);
pub const SOFTBLUE: Color = Color::new(0.66, 0.71, 0.89, 1.0);
pub const SAND: Color = Color::new(0.89, 0.84, 0.66, 1.0);

// TODO: add multithreading and dirty rects
// maybe add brush patterns
// add tile type selection

#[register_app("Sandbox")] // Custom Registration macro
#[derive(Default)]
pub struct Sandbox {
    world_size: (i32, i32),
    selected_tile_type: TileType,
    tile_size: i32,
    brush_size: (i32, i32),
    tiles: Vec<TileType>,
    next_tiles: Vec<TileType>,
    frame_counter: i64,
}

impl GameLoop for Sandbox {
    fn _ready(&mut self) {
        let screen_size = (screen_width(), screen_height());
        self.frame_counter = 0;
        self.tile_size = 8;
        self.selected_tile_type = TileType::Smoke;
        self.world_size = (
            (screen_size.0 / self.tile_size as f32).floor() as i32,
            (screen_size.1 / self.tile_size as f32).floor() as i32,
        );

        self.brush_size = (4, 4);
        let vec_size = (self.world_size.0 * self.world_size.1) as usize;

        self.tiles = vec![TileType::Empty; vec_size];
        self.next_tiles = vec![TileType::Empty; vec_size];
    }

    fn _process_input(&mut self) {
        let pressed_keys = get_keys_pressed();
        for key in &pressed_keys {
            match key {
                KeyCode::Key1 => self.selected_tile_type = TileType::from(1),
                KeyCode::Key2 => self.selected_tile_type = TileType::from(2),
                KeyCode::Key3 => self.selected_tile_type = TileType::from(3),
                KeyCode::Key4 => self.selected_tile_type = TileType::from(4),
                _ => {}
            }
        }

        let (mouse_x, mouse_y) = mouse_position();
        let (screen_w, screen_h) = screen_size();

        //out of screen
        if mouse_x < 0.0 || mouse_x >= screen_w || mouse_y < 0.0 || mouse_y >= screen_h {
            return;
        }

        let grid_x = (mouse_x / self.tile_size as f32).floor() as i32;
        let grid_y = (mouse_y / self.tile_size as f32).floor() as i32;

        let mut tile_type = self.selected_tile_type;
        let mouse_left = is_mouse_button_down(MouseButton::Left);
        let mouse_right = is_mouse_button_down(MouseButton::Right);
        if !mouse_left && !mouse_right {
            return;
        }

        if mouse_right {
            tile_type = TileType::Empty;
        }

        let brush_size = self.brush_size;

        for y in 0..brush_size.1 {
            for x in 0..brush_size.0 {
                let tile_x = grid_x - (brush_size.0 / 2) + x;
                let tile_y = grid_y - (brush_size.1 / 2) + y;

                if !self.in_bounds((tile_x, tile_y)) {
                    continue;
                }

                let idx = self.pos_to_idx((tile_x, tile_y));

                self.tiles[idx] = tile_type;
            }
        }
    }

    fn _update(&mut self, _delta: f32) {
        for y in 0..self.world_size.1 {
            if self.frame_counter % 2 == 0 {
                for x in 0..self.world_size.0 {
                    let idx = self.pos_to_idx((x, y));
                    let tile = self.tiles[idx];
                    if tile == TileType::Empty {
                        continue;
                    }
                    if !self.process_tile(tile, (x, y)) {
                        self.next_tiles[idx] = tile;
                    }
                }
            } else {
                for x in (0..self.world_size.0).rev() {
                    let idx = self.pos_to_idx((x, y));
                    let tile = self.tiles[idx];
                    if tile == TileType::Empty {
                        continue;
                    }
                    if !self.process_tile(tile, (x, y)) {
                        self.next_tiles[idx] = tile;
                    }
                }
            }
        }

        std::mem::swap(&mut self.next_tiles, &mut self.tiles);
        self.next_tiles.fill_with(|| TileType::Empty);

        self.frame_counter = self.frame_counter + 1;
    }

    fn _render(&self) {
        //we draw each tile individually, probably there is better ways to do this
        let tile_size = self.tile_size as f32;
        let len = self.tiles.len();
        for i in 0..len {
            let tile = self.tiles[i];
            if tile == TileType::Empty {
                continue;
            }
            let x = (i % self.world_size.0 as usize) as f32;
            let y = (i / self.world_size.0 as usize) as f32;
            draw_rectangle(
                x * tile_size,
                y * tile_size,
                tile_size,
                tile_size,
                tile.get_color(),
            );
        }

        let screen_size = screen_size();

        let gap = 7;
        let sqr_size = 48;
        let len = TileType::Size as i32 - 1;
        let margin = 9;
        let y = screen_size.1 - 176.0 + sqr_size as f32;
        draw_rectangle_lines(
            32.0,
            y,
            (len * (sqr_size + gap - 2) + margin * 2) as f32,
            (sqr_size + margin * 2) as f32,
            4.0,
            WHITE,
        );

        let mut x = (32 + margin) as f32;
        for i in 0..len {
            let tile_type = TileType::from(i + 1);

            if self.selected_tile_type == tile_type {
                draw_rectangle_lines(
                    x - 3.0,
                    y + margin as f32 - 3.0,
                    sqr_size as f32 + 6.0,
                    sqr_size as f32 + 6.0,
                    6.0,
                    WHITE,
                );
            }

            draw_rectangle(
                x,
                y + margin as f32,
                sqr_size as f32,
                sqr_size as f32,
                tile_type.get_color(),
            );
            let fontsize = sqr_size as f32;
            draw_text(
                format!("{:?}", i + 1),
                x + fontsize / 4.0,
                y + fontsize * 2.0 / 3.0 + margin as f32,
                fontsize,
                BLACK,
            );
            x = x + (sqr_size + gap) as f32;
        }
    }
}

impl Sandbox {
    // we take advantage of the way conditions are validated, the || (or) operator will not check any other option if the first one is true
    fn process_tile(&mut self, tile: TileType, pos: (i32, i32)) -> bool {
        match tile {
            TileType::Sand => self.try_move_down(pos) || self.try_move_down_sides(pos), //,
            TileType::Water => self.try_move_down(pos) || self.try_move_sides(pos),
            TileType::Smoke => {
                self.try_move_up_sides(pos)
                    || self.try_move_up(pos)
                    || self.try_move_sides(pos)
                    || self.try_move_down(pos)
            }
            _ => false,
        }
    }

    // behaviours are quite simple for now, we will probably change this later
    fn try_move_down(&mut self, from: (i32, i32)) -> bool {
        let to = (from.0, from.1 + 1);

        let down = self.is_empty(to);
        if down {
            self.move_tile(from, to)
        }
        down
    }

    fn try_move_down_sides(&mut self, from: (i32, i32)) -> bool {
        let mut down_left =
            self.is_empty((from.0 - 1, from.1 + 1)) && self.is_empty((from.0 - 1, from.1));
        let mut down_right =
            self.is_empty((from.0 + 1, from.1 + 1)) && self.is_empty((from.0 + 1, from.1));

        if down_left && down_right {
            down_right = rand::gen_range(0.0, 1.0) >= 0.5;
            down_left = !down_right;
        }

        if down_right {
            self.move_tile(from, (from.0 + 1, from.1 + 1));
        } else if down_left {
            self.move_tile(from, (from.0 - 1, from.1 + 1));
        }

        down_right || down_left
    }

    fn try_move_up(&mut self, from: (i32, i32)) -> bool {
        let to = (from.0, from.1 - 1);
        let up = self.is_empty(to);
        if up {
            self.move_tile(from, to);
        }
        up
    }

    fn try_move_up_sides(&mut self, from: (i32, i32)) -> bool {
        let mut up_left =
            self.is_empty((from.0 - 1, from.1 - 1)) && self.is_empty((from.0 - 1, from.1));
        let mut up_right =
            self.is_empty((from.0 + 1, from.1 - 1)) && self.is_empty((from.0 + 1, from.1));

        if up_left && up_right {
            up_right = rand::gen_range(0.0, 1.0) >= 0.5;
            up_left = !up_right;
        }

        if up_right {
            self.move_tile(from, (from.0 + 1, from.1 - 1));
        } else if up_left {
            self.move_tile(from, (from.0 - 1, from.1 - 1));
        }

        up_right || up_left
    }

    fn try_move_sides(&mut self, from: (i32, i32)) -> bool {
        let mut left = self.is_empty((from.0 - 1, from.1));
        let mut right = self.is_empty((from.0 + 1, from.1));

        if left && right {
            right = rand::gen_range(0.0, 1.0) >= 0.5;
            left = !right;
        }

        if right {
            self.move_tile(from, (from.0 + 1, from.1));
        } else if left {
            self.move_tile(from, (from.0 - 1, from.1));
        }

        left || right
    }

    fn move_tile(&mut self, from: (i32, i32), to: (i32, i32)) {
        let from_idx = self.pos_to_idx(from);
        let to_idx = self.pos_to_idx(to);
        let tile = self.tiles[from_idx];
        self.next_tiles[to_idx] = tile;
        self.tiles[from_idx] = TileType::Empty;
    }

    #[inline]
    fn is_empty(&self, pos: (i32, i32)) -> bool {
        let idx = self.pos_to_idx(pos);
        self.in_bounds(pos)
            && self.tiles[idx] == TileType::Empty
            && self.next_tiles[idx] == TileType::Empty
    }

    #[inline]
    fn pos_to_idx(&self, pos: (i32, i32)) -> usize {
        (pos.1 * self.world_size.0 + pos.0) as usize
    }

    #[inline]
    fn in_bounds(&self, pos: (i32, i32)) -> bool {
        (pos.0 >= 0) && (pos.0 < self.world_size.0) && (pos.1 >= 0) && (pos.1 < self.world_size.1)
    }
}

#[derive(Default, Clone, PartialEq, Eq, Copy, Debug)]
enum TileType {
    #[default]
    Empty,
    Static,
    Sand,
    Water,
    Smoke,
    Size,
}

impl TileType {
    pub fn get_color(&self) -> Color {
        match *self {
            Self::Empty => EMPTY,
            Self::Static => SOFTBLUE,
            Self::Sand => SAND,
            Self::Water => SKYBLUE,
            Self::Smoke => LIGHTGRAY,
            _ => MAGENTA,
        }
    }
}

impl From<i32> for TileType {
    fn from(item: i32) -> Self {
        match item {
            0 => Self::Empty,
            1 => Self::Static,
            2 => Self::Sand,
            3 => Self::Water,
            4 => Self::Smoke,
            _ => panic!("Invalid color"),
        }
    }
}
