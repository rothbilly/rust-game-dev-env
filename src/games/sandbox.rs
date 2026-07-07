use crate::core::app::AppRegistration; // To register it in the App registry.
use crate::core::game::GameLoop;
use engine_macros::register_app;
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;

pub const EMPTY_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.0);
pub const SOFTBLUE_COLOR: Color = Color::new(0.66, 0.71, 0.89, 1.0);
pub const SAND_COLOR: Color = Color::new(0.89, 0.84, 0.66, 1.0);
pub const GUN_POWDER_COLOR: Color = Color::new(0.122, 0.125, 0.165, 1.0);

// TODO: add multithreading and dirty rects
// maybe add brush patterns
// add tile type selection

const TILE_SIZE: i32 = 4;

#[register_app("Sandbox")] // Custom Registration macro
#[derive(Default)]
pub struct Sandbox {
    world_size: (i32, i32),
    selected_tile_type: u8,
    available_tiles: [u8; 10],
    brush_size: i32,
    tiles: Vec<u8>,
    next_tiles: Vec<u8>,
    frame_counter: i64,
    draw_debug: bool,
}

impl GameLoop for Sandbox {
    fn _ready(&mut self) {
        let screen_size = (screen_width(), screen_height());
        self.frame_counter = 0;
        self.selected_tile_type = SAND;
        self.available_tiles = [
            SAND, WATER, FIRE, GUN_POWDER, WOOD, STEEL, VAPOR, SMOKE, SNOW, LAVA,
        ];

        self.world_size = (
            (screen_size.0 / TILE_SIZE as f32).floor() as i32,
            (screen_size.1 / TILE_SIZE as f32).floor() as i32,
        );

        self.brush_size = 4;
        let vec_size = (self.world_size.0 * self.world_size.1) as usize;

        self.tiles = vec![EMPTY; vec_size];
        self.next_tiles = vec![EMPTY; vec_size];
        self.draw_debug = true;
    }

    fn _process_input(&mut self) {
        let pressed_keys = get_keys_pressed();
        for key in &pressed_keys {
            match key {
                KeyCode::Key1 => self.selected_tile_type = self.available_tiles[0],
                KeyCode::Key2 => self.selected_tile_type = self.available_tiles[1],
                KeyCode::Key3 => self.selected_tile_type = self.available_tiles[2],
                KeyCode::Key4 => self.selected_tile_type = self.available_tiles[3],
                KeyCode::Key5 => self.selected_tile_type = self.available_tiles[4],
                KeyCode::Key6 => self.selected_tile_type = self.available_tiles[5],
                KeyCode::Key7 => self.selected_tile_type = self.available_tiles[6],
                KeyCode::Key8 => self.selected_tile_type = self.available_tiles[7],
                KeyCode::Key9 => self.selected_tile_type = self.available_tiles[8],
                KeyCode::Key0 => self.selected_tile_type = self.available_tiles[9],
                _ => {}
            }
        }

        let mouse_wheel = mouse_wheel();

        self.brush_size = self.brush_size + mouse_wheel.1.round() as i32;
        self.brush_size = self.brush_size.clamp(1, 32);

        let (mouse_x, mouse_y) = mouse_position();

        let (screen_w, screen_h) = screen_size();

        //out of screen
        if mouse_x < 0.0 || mouse_x >= screen_w || mouse_y < 0.0 || mouse_y >= screen_h {
            return;
        }

        let grid_x = (mouse_x / TILE_SIZE as f32).floor() as i32;
        let grid_y = (mouse_y / TILE_SIZE as f32).floor() as i32;

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
                let tile_x = grid_x - (brush_size / 2) + x;
                let tile_y = grid_y - (brush_size / 2) + y;

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
            for y in 0..self.world_size.1 {
                for x in (0..self.world_size.0).rev() {
                    self.check_tile(&x, &y);
                }
            }
        } else {
            for y in 0..self.world_size.1 {
                for x in 0..self.world_size.0 {
                    self.check_tile(&x, &y);
                }
            }
        }

        std::mem::swap(&mut self.next_tiles, &mut self.tiles);
        self.next_tiles.fill_with(|| EMPTY);
        self.frame_counter = self.frame_counter + 1;
    }

    fn _render(&self) {
        //we draw each tile individually, probably there is better ways to do this
        let tile_size = TILE_SIZE as f32;
        let len = self.tiles.len();
        for i in 0..len {
            let tile = self.tiles[i];
            if tile == EMPTY {
                continue;
            }
            let x = (i % self.world_size.0 as usize) as f32;
            let y = (i / self.world_size.0 as usize) as f32;
            draw_rectangle(
                x * tile_size,
                y * tile_size,
                tile_size,
                tile_size,
                get_tile_color(&tile),
            );
        }

        let screen_size = screen_size();

        let gap = 8;
        let sqr_size = 48;
        let len = self.available_tiles.len();
        let margin = 9;
        let y = screen_size.1 - 176.0 + sqr_size as f32;
        draw_rectangle_lines(
            32.0,
            y,
            (len * (sqr_size + gap - 1) + margin * 2) as f32,
            (sqr_size + margin * 2) as f32,
            4.0,
            WHITE,
        );

        let mut x = (32 + margin) as f32;
        let mut i = 0;
        for t in self.available_tiles {
            if self.selected_tile_type == t {
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
                get_tile_color(&t),
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
            i = i + 1;
        }

        let mouse = mouse_position();

        let brush_size = (self.brush_size * TILE_SIZE) as f32;
        draw_rectangle_lines(
            mouse.0 - brush_size / 2.0,
            mouse.1 - brush_size / 2.0,
            brush_size,
            brush_size,
            3.0,
            WHITE,
        );
    }
}

impl Sandbox {
    #[inline]
    fn check_tile(&mut self, x: &i32, y: &i32) {
        let idx = self.pos_to_idx((*x, *y));
        let tile = self.tiles[idx];
        if tile == EMPTY {
            return;
        }
        if !self.process_tile(tile, (*x, *y)) {
            self.next_tiles[idx] = tile;
        }
    }

    fn process_tile(&mut self, tile: u8, pos: (i32, i32)) -> bool {
        let neighbor_indices = self.get_neighbor_indices(pos);

        // we take advantage of the way conditions are validated, the || (or) operator will not check any other option if the first one is true
        match tile {
            SAND => self.try_move_down(pos) || self.try_move_down_sides(pos),
            GUN_POWDER => {
                let mut fire_tiles: i8 = 0;
                let mut n_tile: u8;
                for pos in neighbor_indices {
                    if pos == -1 {
                        continue;
                    }
                    let idx = pos as usize;
                    n_tile = self.tiles[idx];
                    match n_tile {
                        FIRE => {
                            fire_tiles = fire_tiles + 1;
                        }
                        _ => {}
                    }
                }
                if fire_tiles > 0 && self.roll(0.6) {
                    if self.roll(0.6) {
                        self.set_tile(self.pos_to_idx(pos), FIRE);
                    } else {
                        self.set_tile(self.pos_to_idx(pos), SMOKE);
                    }
                    return true;
                }

                self.try_move_down(pos) || self.try_move_down_sides(pos)
            }
            WATER => {
                let mut fire_tiles: i8 = 0;
                let mut n_tile: u8;
                for pos in neighbor_indices {
                    if pos == -1 {
                        continue;
                    }
                    let idx = pos as usize;
                    n_tile = self.tiles[idx];
                    match n_tile {
                        FIRE => {
                            fire_tiles = fire_tiles + 1;
                        }
                        _ => {}
                    }
                }

                if fire_tiles >= 4 {
                    self.set_tile(self.pos_to_idx(pos), VAPOR);
                    return true;
                }

                (self.roll(0.05) && self.try_move_sides(pos))
                    || self.try_move_down(pos)
                    || self.try_move_down_sides(pos)
                    || self.try_move_sides(pos)
            }

            SMOKE => {
                let mut fire_tiles: i8 = 0;
                let mut n_tile: u8;
                for pos in neighbor_indices {
                    if pos == -1 {
                        continue;
                    }
                    let idx = pos as usize;
                    n_tile = self.tiles[idx];
                    match n_tile {
                        FIRE => {
                            fire_tiles = fire_tiles + 1;
                        }
                        _ => {}
                    }
                }

                if fire_tiles >= 3 {
                    self.set_tile(self.pos_to_idx(pos), FIRE);

                    return true;
                }

                (self.roll(0.25) && self.try_move_up_sides(pos))
                    || self.try_move_up(pos)
                    || self.try_move_sides(pos)
                    || self.try_move_down(pos)
            }
            VAPOR => {
                let mut water_tiles: i8 = 0;
                let mut vapor_tiles: i8 = 0;
                let mut n_tile: u8;
                for pos in neighbor_indices {
                    if pos == -1 {
                        continue;
                    }
                    let idx = pos as usize;
                    n_tile = self.tiles[idx];
                    match n_tile {
                        WATER => {
                            water_tiles = water_tiles + 1;
                        }
                        VAPOR => {
                            vapor_tiles = vapor_tiles + 1;
                        }
                        _ => {}
                    }
                }

                if water_tiles >= 4 {
                    self.set_tile(self.pos_to_idx(pos), WATER);
                    return true;
                } else if vapor_tiles >= 3 && self.roll(0.00001) {
                    self.set_tile(self.pos_to_idx(pos), WATER);
                    return true;
                }

                if self.roll(0.0001) {
                    self.set_tile(self.pos_to_idx(pos), EMPTY);
                }

                (self.roll(0.25) && self.try_move_up_sides(pos))
                    || self.try_move_up(pos)
                    || self.try_move_sides(pos)
                    || self.try_move_down(pos)
            }
            SNOW => {
                let mut fire_tiles: i8 = 0;
                let mut water_tiles: i8 = 0;
                let mut vapor_tiles: i8 = 0;
                let mut n_tile: u8;
                for pos in neighbor_indices {
                    if pos == -1 {
                        continue;
                    }
                    let idx = pos as usize;
                    n_tile = self.tiles[idx];
                    match n_tile {
                        FIRE => {
                            fire_tiles = fire_tiles + 1;
                        }
                        WATER => {
                            water_tiles = water_tiles + 1;
                        }
                        VAPOR => {
                            vapor_tiles = vapor_tiles + 1;
                        }
                        _ => {}
                    }
                }

                if fire_tiles >= 3 {
                    self.set_tile(self.pos_to_idx(pos), WATER);

                    return true;
                }
                if water_tiles >= 3 && self.roll(0.01) {
                    self.set_tile(self.pos_to_idx(pos), WATER);
                    return true;
                }
                if vapor_tiles >= 3 && self.roll(0.3) {
                    self.set_tile(self.pos_to_idx(pos), WATER);
                    return true;
                }

                if self.roll(0.10) {
                    let rng = rand::gen_range(0, 100);

                    return match rng {
                        0..50 => self.try_move_down_sides(pos),
                        50..100 => self.try_move_down(pos),
                        _ => false,
                    };
                }
                false
            }

            FIRE => {
                let mut water_tiles: i8 = 0;
                let mut gunpowder_tiles: i8 = 0;
                let mut n_tile: u8;
                for pos in neighbor_indices {
                    if pos == -1 {
                        continue;
                    }
                    let idx = pos as usize;
                    n_tile = self.tiles[idx];
                    match n_tile {
                        WATER | SNOW => {
                            water_tiles = water_tiles + 1;
                        }
                        GUN_POWDER => {
                            gunpowder_tiles = gunpowder_tiles + 1;
                        }
                        _ => {}
                    }
                }

                if water_tiles >= 4 {
                    self.set_tile(self.pos_to_idx(pos), VAPOR);
                    return true;
                } else if gunpowder_tiles > 1 {
                    self.set_tile(self.pos_to_idx(pos), SMOKE);
                    return true;
                }

                if self.roll(0.02) {
                    self.set_tile(self.pos_to_idx(pos), EMPTY);
                    return true;
                }

                let rng = rand::gen_range(0, 100);

                match rng {
                    0..20 => self.try_move_down(pos),
                    20..40 => self.try_move_down_sides(pos),
                    40..60 => self.try_move_sides(pos),
                    60..80 => self.try_move_up_sides(pos),
                    _ => self.try_move_up(pos),
                }
            }
            WOOD => {
                let mut fire_tiles: i8 = 0;
                let mut water_tiles: i8 = 0;
                let mut n_tile: u8;
                for pos in neighbor_indices {
                    if pos == -1 {
                        continue;
                    }
                    let idx = pos as usize;
                    n_tile = self.tiles[idx];
                    match n_tile {
                        FIRE => {
                            fire_tiles = fire_tiles + 1;
                        }
                        WATER => {
                            water_tiles = water_tiles + 1;
                        }
                        _ => {}
                    }
                }

                if fire_tiles > water_tiles && self.roll(0.2) {
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
                let mut fire_tiles: i8 = 0;
                let mut water_tiles: i8 = 0;
                let mut n_tile: u8;
                for pos in neighbor_indices {
                    if pos == -1 {
                        continue;
                    }
                    let idx = pos as usize;
                    n_tile = self.tiles[idx];
                    match n_tile {
                        FIRE => {
                            fire_tiles = fire_tiles + 1;
                        }
                        WATER => {
                            water_tiles = water_tiles + 1;
                        }
                        _ => {}
                    }
                }

                if fire_tiles - water_tiles > 1 {
                    let rng = rand::gen_range(0, 100);
                    match rng {
                        0..20 => self.set_tile(self.pos_to_idx(pos), FIRE),
                        20..40 => self.set_tile(self.pos_to_idx(pos), SMOKE),
                        _ => self.set_tile(self.pos_to_idx(pos), COAL),
                    }
                    return true;
                }
                self.try_move_down(pos) || (self.roll(0.05) && self.try_move_down_sides(pos))
            }
            COAL => {
                let mut fire_tiles: i8 = 0;
                let mut water_tiles: i8 = 0;
                let mut n_tile: u8;
                for pos in neighbor_indices {
                    if pos == -1 {
                        continue;
                    }
                    let idx = pos as usize;
                    n_tile = self.tiles[idx];
                    match n_tile {
                        FIRE => {
                            fire_tiles = fire_tiles + 1;
                        }
                        WATER => {
                            water_tiles = water_tiles + 1;
                        }
                        _ => {}
                    }
                }

                if (fire_tiles - water_tiles) > 0 && self.roll(0.05) {
                    if self.roll(0.4) {
                        self.set_tile(self.pos_to_idx(pos), FIRE);
                    } else {
                        self.set_tile(self.pos_to_idx(pos), SMOKE);
                    }
                    return true;
                }
                self.try_move_down(pos) || (self.roll(0.05) && self.try_move_down_sides(pos))
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
        self.tiles[from_idx] = EMPTY;
    }

    fn get_neighbor_indices(&self, from: (i32, i32)) -> [isize; 8] {
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

        let mut neighbors = [-1; 8];
        let mut i: usize = 0;
        for pos in &positions {
            let n_pos = (from.0 + pos.0, from.1 + pos.1);
            if !self.in_bounds(n_pos) || self.is_empty(n_pos) {
                continue;
            }
            neighbors[i] = self.pos_to_idx(n_pos) as isize;
            i = i + 1;
        }
        neighbors
    }

    #[inline]
    fn set_tile(&mut self, idx: usize, tile_type: u8) {
        self.next_tiles[idx] = tile_type;
    }

    #[inline]
    fn is_empty(&self, pos: (i32, i32)) -> bool {
        let idx = self.pos_to_idx(pos);
        self.in_bounds(pos) && self.tiles[idx] == EMPTY && self.next_tiles[idx] == EMPTY
    }

    #[inline]
    fn pos_to_idx(&self, pos: (i32, i32)) -> usize {
        (pos.1 * self.world_size.0 + pos.0) as usize
    }

    #[inline]
    fn in_bounds(&self, pos: (i32, i32)) -> bool {
        (pos.0 >= 0) && (pos.0 < self.world_size.0) && (pos.1 >= 0) && (pos.1 < self.world_size.1)
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
