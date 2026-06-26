use crate::core::app::AppRegistration; // To register it in the App registry.
use crate::core::game::GameLoop;
use engine_macros::register_app;
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;
use quad_rand::ChooseRandom;

pub const EMPTY: Color = Color::new(0.0, 0.0, 0.0, 0.0);
pub const SOFTBLUE: Color = Color::new(0.66, 0.71, 0.89, 1.0);
pub const SAND: Color = Color::new(0.89, 0.84, 0.66, 1.0);

#[derive(Default)]
#[register_app("Sandbox")] // Custom Registration macro
pub struct Sandbox {
    world_size: (i32, i32),
    selected_tile_type: TileType,
    tile_size: i32,
    brush_size: (i32, i32),
    //next_tiles: Vec<TileType>,
    tiles: Vec<TileType>,
    move_queue: Vec<((i32, i32), (i32, i32))>,
}

impl GameLoop for Sandbox {
    fn _ready(&mut self) {
        let screen_size = (screen_width(), screen_height());
        self.tile_size = 4;
        self.selected_tile_type = TileType::Sand;
        self.world_size = (
            (screen_size.0 / self.tile_size as f32).floor() as i32,
            (screen_size.1 / self.tile_size as f32).floor() as i32,
        );

        self.brush_size = (16, 16);
        let vec_size = (self.world_size.0 * self.world_size.1) as usize;

        //self.next_tiles = vec![TileType::Empty; vec_size];
        self.tiles = vec![TileType::Empty; vec_size];
        self.move_queue = Vec::new();
        self.move_queue.reserve(vec_size);
    }

    fn _process_input(&mut self) {
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
            for x in 0..self.world_size.0 {
                let idx = self.pos_to_idx((x, y));
                let tile = self.tiles[idx];
                if tile == TileType::Empty {
                    continue;
                }
                if !self.process_tile(&tile, (x, y)) {
                    self.tiles[idx] = tile;
                }
            }
        }

        //self.move_queue.sort_by_key(|pair| pair.0);
        self.move_queue.shuffle();
        self.move_queue.dedup_by_key(|k| k.0);

        let size = self.move_queue.len();
        for i in 0..size {
            let mov = self.move_queue[i];
            self.move_tile(mov.1, mov.0);
        }
        self.move_queue.clear();

        // let move_queue = &self.move_queue;
        // let size = move_queue.len();
        // let idx = 0;
        // for i in 0..size - 1 {
        //     let next_dest = move_queue[i + 1].0;
        // }

        //swap and clear
        //std::mem::swap(&mut self.tiles, &mut self.next_tiles);
        //self.next_tiles.fill_with(|| TileType::Empty);
    }

    fn _render(&self) {
        //clear_background(BLACK);

        let tile_size = self.tile_size as f32;
        let mouse_screen = mouse_position();
        let mouse_tile: (i32, i32) = (
            (mouse_screen.0 / tile_size).floor() as i32,
            (mouse_screen.1 / tile_size).floor() as i32,
        );

        draw_text(
            std::format!("mouse tile: {:?}", mouse_tile),
            32.0,
            64.0,
            32.0,
            WHITE,
        );

        let tile_size = self.tile_size as f32;
        let len = self.tiles.len();
        for i in 0..len {
            //let idx = i as usize;
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
    }
}

impl Sandbox {
    fn process_tile(&mut self, tile: &TileType, pos: (i32, i32)) -> bool {
        match tile {
            TileType::Sand => self.try_move_down(pos) || self.try_move_down_sides(pos), //,
            TileType::Water => {
                self.try_move_down(pos) || self.try_move_down_sides(pos) || self.try_move_sides(pos)
            }
            TileType::Smoke => {
                self.try_move_up_sides(pos) || self.try_move_up(pos) || self.try_move_sides(pos)
            }
            _ => false,
        }
    }

    fn try_move_down(&mut self, from: (i32, i32)) -> bool {
        let to = (from.0, from.1 + 1);

        let down = self.is_empty(to);
        if down {
            self.queue_move(from, to)
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
            self.queue_move(from, (from.0 + 1, from.1 + 1));
        } else if down_left {
            self.queue_move(from, (from.0 - 1, from.1 + 1));
        }

        down_right || down_left
    }

    fn try_move_up(&mut self, from: (i32, i32)) -> bool {
        let to = (from.0, from.1 - 1);
        let up = self.is_empty(to);
        if up {
            self.queue_move(from, to);
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
            self.queue_move(from, (from.0 + 1, from.1 - 1));
        } else if up_left {
            self.queue_move(from, (from.0 - 1, from.1 - 1));
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
            self.queue_move(from, (from.0 + 1, from.1));
        } else if left {
            self.queue_move(from, (from.0 - 1, from.1));
        }

        left || right
    }

    #[inline]
    fn move_tile(&mut self, from: (i32, i32), to: (i32, i32)) {
        let from_idx = self.pos_to_idx(from);
        let to_idx = self.pos_to_idx(to);

        let tile = self.tiles[from_idx];
        self.tiles[to_idx] = tile;
        self.tiles[from_idx] = TileType::Empty;
    }

    #[inline]
    fn queue_move(&mut self, from: (i32, i32), to: (i32, i32)) {
        self.move_queue.push((to, from));
    }

    #[inline]
    fn is_empty(&self, pos: (i32, i32)) -> bool {
        self.in_bounds(pos) && (self.tiles[self.pos_to_idx(pos)] == TileType::Empty)
        //&& (self.next_tiles[self.pos_to_idx(pos)] == TileType::Empty)
    }

    #[inline]
    fn pos_to_idx(&self, pos: (i32, i32)) -> usize {
        // y * width + x
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
