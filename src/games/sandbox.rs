use crate::core::app::AppRegistration; // To register it in the App registry.
use crate::core::game::GameLoop;
use engine_macros::register_app;
use macroquad::prelude::*;
use macroquad::ui::{
    Drag, Ui, hash, root_ui,
    widgets::{self, Group},
};

pub const EMPTY_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.0);
pub const SOFTBLUE_COLOR: Color = Color::new(0.66, 0.71, 0.89, 1.0);
pub const SAND_COLOR: Color = Color::new(0.89, 0.84, 0.66, 1.0);
pub const GUN_POWDER_COLOR: Color = Color::new(0.122, 0.125, 0.165, 1.0);

// Screen references
// Snes NTSC 256 * 224 @60Hz PAL 256 * 240 @50Hz
// GBA 240 * 160 3:2 @60Hz
const WORLD_WIDTH: i32 = 256;
const WORLD_HEIGHT: i32 = 240;
const ASPECT_RATIO: f32 = WORLD_WIDTH as f32 / WORLD_HEIGHT as f32;

/*
    IMPORTANT: improve single thread performance before adding multithreading
    TODO: add spatial partitioning (chunks) and dirty rects
    TODO: add UI back
    maybe add brush patterns
*/
#[register_app("Sandbox")] // Custom Registration macro
pub struct Sandbox {
    brush_size: i16,
    max_brush_size: i16,
    tiles: Vec<u8>,
    next_tiles: Vec<u8>,
    frame_counter: u64,
    world_image: Image,
    world_texture: Texture2D,
    camera: Camera2D,
    draw_debug: bool,
    inventory_data: Data,
}

impl Default for Sandbox {
    fn default() -> Self {
        Self {
            brush_size: 4,
            max_brush_size: 64,
            tiles: Vec::<u8>::new(),
            next_tiles: Vec::<u8>::new(),
            frame_counter: 0,
            world_image: Image::empty(),
            world_texture: Texture2D::empty(),
            camera: Camera2D::default(),
            draw_debug: true,
            inventory_data: Data::new(),
            base: Default::default(),
        }
    }
}

impl GameLoop for Sandbox {
    fn _ready(&mut self) {
        let vec_size = (WORLD_WIDTH * WORLD_HEIGHT) as usize;
        self.tiles = vec![EMPTY; vec_size];
        self.next_tiles = vec![EMPTY; vec_size];
        self.world_image = Image::gen_image_color(WORLD_WIDTH as u16, WORLD_HEIGHT as u16, BLACK);
        self.world_texture = Texture2D::from_image(&self.world_image);
        self.world_texture.set_filter(FilterMode::Nearest);
    }

    fn _process_input(&mut self) {
        let pressed_keys = get_keys_pressed();
        for key in &pressed_keys {
            let key_value = *key as i32;
            let idx = key_value - KeyCode::Key0 as i32;
            if (1..=9).contains(&idx) {
                self.inventory_data.selected = (idx - 1) as usize;
            } else if idx == 0 {
                self.inventory_data.selected = 9;
            }
            if *key == KeyCode::Tab {
                self.inventory_data.palette_visible = !self.inventory_data.palette_visible;
            }
        }

        self.process_ui();

        if self.inventory_data.item_dragging {
            return;
        }

        let mouse_wheel = mouse_wheel();

        self.brush_size += (mouse_wheel.1.round() * 2.0) as i16;
        self.brush_size = self.brush_size.clamp(1, self.max_brush_size);

        let mouse_left = is_mouse_button_down(MouseButton::Left);
        let mouse_right = is_mouse_button_down(MouseButton::Right);
        if !mouse_left && !mouse_right {
            return;
        }

        if mouse_right {
            self.paint_tiles(EMPTY);
            return;
        }

        if let Some(tile_type) = self.inventory_data.get_selected_tile() {
            self.paint_tiles(tile_type);
        }
    }

    fn _update(&mut self, _delta: f32) {
        if self.frame_counter.is_multiple_of(2) {
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
            2.0,
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
        let screen_size = (screen_width(), screen_height());
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

    fn process_ui(&mut self) {
        // draw ui

        // hotbar

        let slot_size = 56.0;
        let hotbar_width = slot_size * 10.0 + 2.625 * 10.0;
        let hotbar_height = slot_size + 20.0;
        widgets::Window::new(
            hash!(),
            vec2(32.0, screen_height() - hotbar_height - 32.0),
            vec2(hotbar_width, hotbar_height),
        )
        .label("")
        .titlebar(true)
        .ui(&mut root_ui(), |ui| {
            self.inventory_data.hot_bar(ui);
        });

        // tile palette
        //
        let slot_size = 64.0;
        let inv_width = slot_size * 2.0 + 8.0;
        let inv_height = slot_size * 5.0 + 8.0;
        if self.inventory_data.palette_visible {
            widgets::Window::new(
                hash!(),
                vec2(32.0, screen_height() / 2.0),
                vec2(inv_width, inv_height),
            )
            .label("Tile Palette")
            .titlebar(true)
            .ui(&mut root_ui(), |ui| {
                self.inventory_data.tile_palette(ui);
            });
        }

        match self.inventory_data.fit_command.take() {
            Some(FittingCommand::Unfit { target_slot }) => {
                self.inventory_data.set_item(target_slot, None)
            }
            Some(FittingCommand::Fit { target_slot, item }) => {
                self.inventory_data.set_item(target_slot, Some(item));
            }
            Some(FittingCommand::Refit {
                target_slot,
                origin_slot,
            }) => {
                let mut origin_item: Option<u8> = None;
                for i in 0..self.inventory_data.hot_bar.len() {
                    if self.inventory_data.hot_bar[i].1.id == origin_slot {
                        origin_item = Some(self.inventory_data.hot_bar[i].1.item.unwrap());
                        break;
                    }
                }
                self.inventory_data.set_item(target_slot, origin_item);
                self.inventory_data.set_item(origin_slot, None);
            }
            None => {}
        }
    }

    fn paint_tiles(&mut self, tile_type: u8) {
        let mouse = self.mouse_to_world();

        let grid_x = mouse.x.floor() as i16;
        let grid_y = mouse.y.floor() as i16;

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
                if (neighbors[EMPTY as usize] >= 3 && self.roll(0.001))
                    || (neighbors[FIRE as usize] >= 2 && self.roll(0.1))
                {
                    self.set_tile(self.pos_to_idx(pos), VAPOR);
                    return true;
                }

                (self.roll(0.05) && self.try_move_sides(pos))
                    || self.try_move(pos, (pos.0, pos.1 + 1))
                    || self.try_move_down_sides(pos)
                    || self.try_move_sides(pos)
            }

            SMOKE => {
                if self.roll(0.0002) {
                    self.set_tile(self.pos_to_idx(pos), EMPTY);
                    return true;
                }

                if neighbors[FIRE as usize] >= 2 && self.roll(0.6) {
                    self.set_tile(self.pos_to_idx(pos), FIRE);
                    return true;
                }

                (self.roll(0.25) && self.try_move_up_sides(pos))
                    || self.try_move(pos, (pos.0, pos.1 - 1))
                    || self.try_move_sides(pos)
                    || self.try_move(pos, (pos.0, pos.1 + 1))
            }
            VAPOR => {
                if self.roll(0.0002) {
                    self.set_tile(self.pos_to_idx(pos), EMPTY);
                    return true;
                }

                if neighbors[WATER as usize] >= 4
                    || (neighbors[VAPOR as usize] >= 3 && self.roll(0.000001))
                {
                    self.set_tile(self.pos_to_idx(pos), WATER);
                    return true;
                }

                (self.roll(0.25) && self.try_move_up_sides(pos))
                    || self.try_move(pos, (pos.0, pos.1 - 1))
                    || self.try_move_sides(pos)
                    || self.try_move(pos, (pos.0, pos.1 + 1))
            }
            SNOW => {
                if neighbors[FIRE as usize] >= 1
                    || (neighbors[WATER as usize] >= 3 && self.roll(0.01))
                    || (neighbors[VAPOR as usize] >= 2 && self.roll(0.3))
                {
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
                if neighbors[GUN_POWDER as usize] > 1 {
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
                        self.set_tile(self.pos_to_idx(pos), BURNT_WOOD);
                    } else {
                        self.set_tile(self.pos_to_idx(pos), SMOKE);
                    }
                    return true;
                }
                false
            }
            BURNT_WOOD => {
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

        let mut neighbors = [0_i8; TILE_TYPES];

        let mut tile: u8;
        for pos in &positions {
            let n_pos = (from.0 + pos.0, from.1 + pos.1);

            if !self.in_bounds(n_pos) {
                continue;
            }

            tile = self.tiles[self.pos_to_idx(n_pos)];

            neighbors[tile as usize] += 1;
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
            (0.0..=1.0).contains(&chance),
            "Chance for a roll must be equals or less than 1.0 and more than 0.0"
        );
        rand::gen_range(0.0, 1.0) < chance
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
const BURNT_WOOD: u8 = 8;
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
        BURNT_WOOD => DARKBROWN,
        COAL => DARKGRAY,
        FIRE => ORANGE,
        GUN_POWDER => GUN_POWDER_COLOR,
        LAVA => RED,
        _ => MAGENTA,
    }
}

pub struct Slot {
    id: u64,
    item: Option<u8>,
}
impl Slot {
    fn new(id: u64) -> Slot {
        Slot { id, item: None }
    }

    fn new_with(id: u64, item: u8) -> Slot {
        Slot {
            id,
            item: Some(item),
        }
    }
}

enum FittingCommand {
    Unfit { target_slot: u64 },
    Fit { target_slot: u64, item: u8 },
    Refit { target_slot: u64, origin_slot: u64 },
}

struct Data {
    hot_bar: [(&'static str, Slot); 10],
    tile_palette: Vec<u8>,
    palette_visible: bool,
    item_dragging: bool,
    selected: usize,
    fit_command: Option<FittingCommand>,
    tile_image: Image,
}

impl Data {
    pub fn new() -> Data {
        Data {
            hot_bar: {
                [
                    ("1", Slot::new_with(hash!(), SAND)),
                    ("2", Slot::new_with(hash!(), WATER)),
                    ("3", Slot::new_with(hash!(), FIRE)),
                    ("4", Slot::new(hash!())),
                    ("5", Slot::new(hash!())),
                    ("6", Slot::new(hash!())),
                    ("7", Slot::new(hash!())),
                    ("8", Slot::new(hash!())),
                    ("9", Slot::new(hash!())),
                    ("0", Slot::new(hash!())),
                ]
            },
            tile_palette: {
                let mut vec = Vec::<u8>::with_capacity(TILE_TYPES - 1);
                for i in 1..TILE_TYPES {
                    vec.push(i as u8);
                }
                vec
            },
            palette_visible: true,
            item_dragging: false,
            selected: 0,
            fit_command: None,
            tile_image: Image::gen_image_color(1, 1, WHITE),
        }
    }

    fn get_selected_tile(&self) -> Option<u8> {
        self.hot_bar[self.selected].1.item
    }

    fn hot_bar(&mut self, ui: &mut Ui) {
        let fit_command = &mut self.fit_command;
        let item_dragging = &mut self.item_dragging;
        let slot_size = 52.0;

        for i in 0..self.hot_bar.len() {
            let (label, slot) = &self.hot_bar[i];
            let drag = Group::new(slot.id, vec2(slot_size + 4.0, slot_size + 4.0))
                .draggable(slot.item.is_some())
                .hoverable(*item_dragging)
                .highlight(*item_dragging)
                .ui(ui, |ui| {
                    if let Some(ref item) = slot.item {
                        self.tile_image.set_pixel(0, 0, get_tile_color(item));
                        ui.texture(
                            Texture2D::from_image(&self.tile_image),
                            slot_size,
                            slot_size,
                        );
                    } else {
                        ui.label(vec2(24.0, 24.0), label);
                    }
                });

            match drag {
                Drag::Dropped(_, Some(id)) if slot.item.is_some() => {
                    *fit_command = Some(FittingCommand::Refit {
                        target_slot: (id),
                        origin_slot: (slot.id),
                    });
                }
                Drag::Dropped(_, None) if slot.item.is_some() => {
                    *fit_command = Some(FittingCommand::Unfit {
                        target_slot: (slot.id),
                    });
                }
                Drag::Dropped(_, _) => unreachable!(),
                Drag::Dragging(_pos, _id) => {
                    //debug!("slots pos: {:?}, id: {:?}", pos, id);
                    *item_dragging = true;
                }
                Drag::No => {}
            }
        }
    }

    fn tile_palette(&mut self, ui: &mut Ui) {
        let item_dragging = &mut self.item_dragging;
        let slot_size = 54.0;

        for (n, item) in self.tile_palette.iter().enumerate() {
            let drag = Group::new(
                hash!("tile_palette", n),
                vec2(slot_size + 4., slot_size + 4.),
            )
            .draggable(true)
            .ui(ui, |ui| {
                self.tile_image.set_pixel(0, 0, get_tile_color(item));
                ui.texture(
                    Texture2D::from_image(&self.tile_image),
                    slot_size,
                    slot_size,
                );
            });

            match drag {
                Drag::Dropped(_, Some(id)) => {
                    self.fit_command = Some(FittingCommand::Fit {
                        target_slot: (id),
                        item: (*item),
                    });
                    *item_dragging = false;
                }
                Drag::Dropped(_, _) => {
                    *item_dragging = false;
                }
                Drag::Dragging(_pos, _id) => {
                    //debug!("tile palette pos: {:?}, id: {:?}", pos, id);
                    *item_dragging = true;
                }
                _ => {}
            }
        }
    }
    fn set_item(&mut self, id: u64, item: Option<u8>) {
        for i in 0..self.hot_bar.len() {
            if self.hot_bar[i].1.id == id {
                self.hot_bar[i].1.item = item;
                break;
            }
        }
    }
}
