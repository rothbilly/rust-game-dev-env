use crate::core::app::AppRegistration; // To register it in the App registry.
use crate::core::game::GameLoop;
use engine_macros::register_app;
use macroquad::prelude::*;
use macroquad::ui::{
    Drag, Ui, hash, root_ui,
    widgets::{self, Group},
};

// Screen references
// Snes NTSC 256 * 224 @60Hz PAL 256 * 240 @50Hz
// GBA 240 * 160 3:2 @60Hz

// WORLD SIZE has to be square in order to avoid uv streching
const WORLD_WIDTH: i32 = 512;
const WORLD_HEIGHT: i32 = 512;
const ASPECT_RATIO: f32 = WORLD_WIDTH as f32 / WORLD_HEIGHT as f32;
const ATLAS_TEXTURES_SIZE: i32 = 64;
const ATLAS_BYTES: &[u8] = include_bytes!("../../assets/sprites/sandbox/tile_atlas.png");
const BACKGROUND_BYTES: &[u8] = include_bytes!("../../assets/sprites/sandbox/background1.png");

/*
   IMPORTANT: improve single thread performance before adding multithreading
   TODO: save & load tiles to/from disk (maybe bitmap? png? or just binary?)
   TODO: implement Command/Action Design pattern to enable Undo/Redo
   TODO: maybe add brush patterns
   TODO: improve "compressed" data layout ??
   TODO: add spatial partitioning (chunks)
   TODO: decouple dirty tiles from tiles to refresh ( gotta back to single buffer? T_T )
   TODO: maybe add multithreading??
**/

#[register_app("Sandbox")] // Custom Registration macro
pub struct Sandbox {
    brush_size: i16,
    max_brush_size: i16,
    tiles: Vec<u8>,
    next_tiles: Vec<u8>,
    frame_counter: u64,
    inventory: UIData,
    camera: Camera2D,
    render_data: RenderData,
    game_state: GameState,
    dirty_area: DirtyRect,
}

#[derive(Default, PartialEq, Eq)]
enum GameState {
    #[default]
    Paused,
    Running,
}

impl Default for Sandbox {
    fn default() -> Self {
        let vec_size = (WORLD_WIDTH * WORLD_HEIGHT) as usize;
        Self {
            game_state: GameState::Running,
            brush_size: 4,
            max_brush_size: 48,
            tiles: vec![Tile::Empty as u8; vec_size],
            next_tiles: vec![Tile::Empty as u8; vec_size],
            frame_counter: 0,
            inventory: UIData::new(),
            render_data: RenderData::default(),
            camera: Camera2D::default(),
            base: Default::default(),
            dirty_area: DirtyRect::default(),
        }
    }
}

impl GameLoop for Sandbox {
    fn _ready(&mut self) {
        self.render_data.world_material = Some(
            load_material(
                ShaderSource::Glsl {
                    vertex: TILES_VERTEX_SHADER,
                    fragment: TILES_FRAGMENT_SHADER,
                },
                MaterialParams {
                    uniforms: vec![
                        UniformDesc::new("atlas_textures_size", UniformType::Int2),
                        UniformDesc::new("grid_size", UniformType::Float2),
                    ],
                    textures: vec![
                        "atlas_texture".to_owned(),
                        "tiles".to_owned(),
                        "background".to_owned(),
                    ],
                    ..Default::default()
                },
            )
            .unwrap(),
        );

        let atlas_img = Image::from_file_with_format(ATLAS_BYTES, Some(ImageFormat::Png)).unwrap();
        self.inventory.tiles_atlas = Some(atlas_img.clone());
        self.inventory.init_textures();

        if let Some(mat) = &self.render_data.world_material {
            self.render_data.tiles_atlas = Texture2D::from_image(&atlas_img);
            self.render_data.tiles_atlas.set_filter(FilterMode::Nearest);
            self.render_data.tiles = Texture2D::from_rgba8(
                WORLD_WIDTH as u16 / 4,
                WORLD_HEIGHT as u16,
                &self.next_tiles,
            );
            self.render_data.tiles.set_filter(FilterMode::Nearest);
            self.render_data.background =
                Texture2D::from_file_with_format(BACKGROUND_BYTES, Some(ImageFormat::Png));
            mat.set_uniform(
                "atlas_textures_size",
                ivec2(ATLAS_TEXTURES_SIZE, ATLAS_TEXTURES_SIZE),
            );
            mat.set_uniform("grid_size", vec2(WORLD_WIDTH as f32, WORLD_HEIGHT as f32));
            mat.set_texture("atlas_texture", self.render_data.tiles_atlas.clone());
            mat.set_texture("tiles", self.render_data.tiles.clone());
            mat.set_texture("background", self.render_data.background.clone());
        }
    }

    fn _process_input(&mut self) {
        let pressed_keys = get_keys_pressed();
        for key in &pressed_keys {
            let key_value = *key as i32;
            let idx = key_value - KeyCode::Key0 as i32;
            if (1..=9).contains(&idx) {
                self.inventory.selected = (idx - 1) as usize;
            } else if idx == 0 {
                self.inventory.selected = 9;
            }
            match key {
                KeyCode::Space => {
                    if self.game_state == GameState::Paused {
                        self.game_state = GameState::Running
                    } else {
                        self.game_state = GameState::Paused
                    }
                }
                KeyCode::Tab => {
                    self.inventory.palette_visible = !self.inventory.palette_visible;
                }
                _ => {}
            }
        }

        self.process_ui();

        if self.inventory.item_dragging {
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
            self.paint_tiles(Tile::Empty);
            return;
        }

        if let Some(tile_type) = self.inventory.get_selected_tile() {
            self.paint_tiles(tile_type);
        }
    }

    fn _update(&mut self, _delta: f32) {
        match self.game_state {
            GameState::Running => 'update: {
                if self.dirty_area.is_empty() {
                    break 'update;
                }

                let start = (self.dirty_area.x, self.dirty_area.y);
                let end = (self.dirty_area.x2, self.dirty_area.y2);

                self.dirty_area = DirtyRect::default();

                if self.frame_counter.is_multiple_of(2) {
                    for y in start.1..end.1 {
                        for x in (start.0..end.0).rev() {
                            self.check_tile(&x, &y);
                        }
                    }
                } else {
                    for y in start.1..end.1 {
                        for x in start.0..end.0 {
                            self.check_tile(&x, &y);
                        }
                    }
                }

                std::mem::swap(&mut self.next_tiles, &mut self.tiles);
                self.next_tiles.fill_with(|| Tile::Empty as u8);
            }
            _ => {}
        }

        if let Some(material) = &self.render_data.world_material {
            self.render_data.tiles.update_from_bytes(
                WORLD_WIDTH as u32 / 4,
                WORLD_HEIGHT as u32,
                &self.tiles,
            );
            material.set_texture("tiles", self.render_data.tiles.clone());
        }

        self.update_camera();

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
            self.inventory.hot_bar(ui);
        });

        // tile palette
        //
        let slot_size = 64.0;
        let inv_width = slot_size * 2.0 + 8.0;
        let inv_height = slot_size * 5.0 + 8.0;
        if self.inventory.palette_visible {
            widgets::Window::new(
                hash!(),
                vec2(32.0, screen_height() / 2.0),
                vec2(inv_width, inv_height),
            )
            .label("Tile Palette")
            .titlebar(true)
            .ui(&mut root_ui(), |ui| {
                self.inventory.tile_palette(ui);
            });
        }

        self.frame_counter = (self.frame_counter + 1) % 60;
    }

    fn _render(&self) {
        set_camera(&self.camera);

        if let Some(mat) = &self.render_data.world_material {
            gl_use_material(mat);
            draw_rectangle(0.0, 0.0, WORLD_WIDTH as f32, WORLD_WIDTH as f32, WHITE);
            gl_use_default_material();
        }

        let mut mouse = self.mouse_to_world();
        mouse.x = mouse.x.floor();
        mouse.y = mouse.y.floor();
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

    fn _debug_draw(&self) {
        let area = &self.dirty_area;

        if !area.is_empty() {
            let size = (area.x2 - area.x, area.y2 - area.y);
            draw_rectangle_lines(
                area.x as f32,
                area.y as f32,
                size.0 as f32,
                size.1 as f32,
                5.0,
                RED,
            );
        }

        draw_text(format!("{:?}", area), 32.0, 32.0, 12.0, WHITE);
        draw_text(
            format!("Mouse: {:?}", mouse_position()),
            32.0,
            48.0,
            12.0,
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
        match self.inventory.fit_command.take() {
            Some(FittingCommand::Unfit { target_slot }) => {
                self.inventory.set_item(target_slot, None)
            }
            Some(FittingCommand::Fit { target_slot, item }) => {
                self.inventory.set_item(target_slot, Some(item));
            }
            Some(FittingCommand::Refit {
                target_slot,
                origin_slot,
            }) => {
                let mut origin_item: Option<Tile> = None;
                let mut target_item: Option<Tile> = None;
                for i in 0..self.inventory.hot_bar.len() {
                    if self.inventory.hot_bar[i].1.id == origin_slot {
                        origin_item = self.inventory.hot_bar[i].1.item;
                    }
                    if self.inventory.hot_bar[i].1.id == target_slot {
                        target_item = self.inventory.hot_bar[i].1.item;
                    }
                }
                self.inventory.set_item(target_slot, origin_item);
                self.inventory.set_item(origin_slot, target_item);
            }
            None => {}
        }
    }

    fn paint_tiles(&mut self, tile_type: Tile) {
        let mouse = self.mouse_to_world();
        let mouse = vec2(mouse.x.floor(), mouse.y.floor());

        let grid_x = mouse.x as i16;
        let grid_y = mouse.y as i16;

        let brush_size = self.brush_size;

        for y in 0..brush_size {
            for x in 0..brush_size {
                let tile_x = grid_x - (brush_size / 2) + x;
                let tile_y = grid_y - (brush_size / 2) + y;

                if !self.in_bounds((tile_x, tile_y)) {
                    continue;
                }
                self.set_tile((tile_x, tile_y), tile_type, true);
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
        let tile: Tile = self.tiles[idx].into();
        if tile == Tile::Empty {
            return;
        }
        if !self.process_tile(tile, (*x, *y)) {
            self.next_tiles[idx] = tile as u8;
            self.dirty_area.mark_dirty((*x, *y));
        }
    }

    fn process_tile(&mut self, tile: Tile, pos: (i16, i16)) -> bool {
        let neighbors = self.get_neighbors_per_type(pos);

        // we take advantage of the way conditions are validated, the || (or) operator will not check any other option if the first one is true
        match tile {
            Tile::Sand => self.try_move(pos, (pos.0, pos.1 + 1)) || self.try_move_down_sides(pos),
            Tile::GunPowder => {
                if (neighbors[Tile::Fire as usize] > 0 || neighbors[Tile::Lava as usize] > 0)
                    && self.roll(0.6)
                {
                    if self.roll(0.6) {
                        self.set_tile(pos, Tile::Fire, false);
                    } else {
                        self.set_tile(pos, Tile::Smoke, false);
                    }
                    return true;
                }

                self.try_move(pos, (pos.0, pos.1 + 1)) || self.try_move_down_sides(pos)
            }
            Tile::Water => {
                if (neighbors[Tile::Empty as usize] >= 3 && self.roll(0.001))
                    || ((neighbors[Tile::Fire as usize] >= 2 || neighbors[Tile::Lava as usize] > 0)
                        && self.roll(0.1))
                {
                    self.set_tile(pos, Tile::Vapor, false);
                    return true;
                }

                (self.roll(0.05) && self.try_move_sides(pos))
                    || self.try_move(pos, (pos.0, pos.1 + 1))
                    || self.try_move_down_sides(pos)
                    || self.try_move_sides(pos)
            }

            Tile::Smoke => {
                if self.roll(0.0002) {
                    self.set_tile(pos, Tile::Empty, false);
                    return true;
                }

                if (neighbors[Tile::Fire as usize] >= 2 || neighbors[Tile::Lava as usize] > 0)
                    && self.roll(0.6)
                {
                    self.set_tile(pos, Tile::Fire, false);
                    return true;
                }

                (self.roll(0.25) && self.try_move_up_sides(pos))
                    || self.try_move(pos, (pos.0, pos.1 - 1))
                    || self.try_move_sides(pos)
                    || self.try_move(pos, (pos.0, pos.1 + 1))
            }
            Tile::Vapor => {
                if self.roll(0.0002) {
                    self.set_tile(pos, Tile::Empty, false);
                    return true;
                }

                if neighbors[Tile::Water as usize] >= 4
                    || (neighbors[Tile::Vapor as usize] >= 3 && self.roll(0.000001))
                {
                    self.set_tile(pos, Tile::Water, false);
                    return true;
                }

                (self.roll(0.25) && self.try_move_up_sides(pos))
                    || self.try_move(pos, (pos.0, pos.1 - 1))
                    || self.try_move_sides(pos)
                    || self.try_move(pos, (pos.0, pos.1 + 1))
            }
            Tile::Snow => {
                if neighbors[Tile::Fire as usize] > 1
                    || neighbors[Tile::Lava as usize] > 0
                    || (neighbors[Tile::Water as usize] >= 3 && self.roll(0.01))
                    || (neighbors[Tile::Vapor as usize] >= 2 && self.roll(0.3))
                {
                    self.set_tile(pos, Tile::Water, false);
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

            Tile::Fire => {
                if neighbors[Tile::GunPowder as usize] > 1 {
                    self.set_tile(pos, Tile::Smoke, false);
                    return true;
                }

                if self.roll(0.02) {
                    self.set_tile(pos, Tile::Empty, false);
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
            Tile::Wood => {
                if (neighbors[Tile::Fire as usize] + neighbors[Tile::Lava as usize])
                    > neighbors[Tile::Water as usize]
                    && self.roll(0.2)
                {
                    if self.roll(0.7) {
                        self.set_tile(pos, Tile::BurntWood, false);
                    } else {
                        self.set_tile(pos, Tile::Smoke, false);
                    }
                    return true;
                }
                false
            }
            Tile::BurntWood => {
                if (neighbors[Tile::Fire as usize] + neighbors[Tile::Lava as usize])
                    - neighbors[Tile::Water as usize]
                    > 1
                {
                    let rng = rand::gen_range(0, 100);
                    match rng {
                        0..20 => self.set_tile(pos, Tile::Fire, false),
                        20..40 => self.set_tile(pos, Tile::Smoke, false),
                        _ => self.set_tile(pos, Tile::Coal, false),
                    }
                    return true;
                }
                self.try_move(pos, (pos.0, pos.1 + 1))
                    || (self.roll(0.05) && self.try_move_down_sides(pos))
            }
            Tile::Leaf => {
                if neighbors[Tile::Fire as usize] > 1 || neighbors[Tile::Lava as usize] > 0 {
                    self.set_tile(pos, Tile::Fire, false);
                    return true;
                }

                if (neighbors[Tile::Leaf as usize] + neighbors[Tile::Wood as usize] > 0)
                    && self.roll(0.99)
                {
                    return false;
                }

                self.roll(0.1) && (self.try_move_down_sides(pos) || self.try_move_sides(pos))
            }
            Tile::Coal => {
                if ((neighbors[Tile::Fire as usize] + neighbors[Tile::Lava as usize])
                    - neighbors[Tile::Water as usize])
                    > 0
                    && self.roll(0.05)
                {
                    if self.roll(0.4) {
                        self.set_tile(pos, Tile::Fire, false);
                    } else {
                        self.set_tile(pos, Tile::Smoke, false);
                    }
                    return true;
                }
                self.try_move(pos, (pos.0, pos.1 + 1))
                    || (self.roll(0.05) && self.try_move_down_sides(pos))
            }
            Tile::Lava => {
                self.roll(0.2)
                    && (self.try_move(pos, (pos.0, pos.1 + 1)) || self.try_move_down_sides(pos))
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
        let tile = Tile::from(self.tiles[from_idx]);
        self.set_tile(to, tile, false);
        self.set_tile(from, Tile::Empty, false);
    }

    // the index is the type
    fn get_neighbors_per_type(&self, from: (i16, i16)) -> [i8; Tile::Size as usize] {
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

        let mut neighbors = [0_i8; Tile::Size as usize];

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
    fn set_tile(&mut self, pos: (i16, i16), tile_type: Tile, back: bool) {
        let idx: usize = self.pos_to_idx(pos);
        self.set_tile_by_idx(idx, tile_type, back);
        let mark_dirty = match tile_type {
            Tile::Empty => false,
            _ => true,
        };

        if mark_dirty {
            self.dirty_area.mark_dirty(pos);
        }
    }

    #[inline]
    fn set_tile_by_idx(&mut self, idx: usize, tile_type: Tile, back: bool) {
        if back {
            self.tiles[idx] = tile_type as u8;
            return;
        }
        self.next_tiles[idx] = tile_type as u8;
    }

    #[inline]
    fn is_empty(&self, pos: (i16, i16)) -> bool {
        let idx = self.pos_to_idx(pos);
        self.in_bounds(pos)
            && self.tiles[idx] == (Tile::Empty as u8)
            && self.next_tiles[idx] == (Tile::Empty as u8)
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

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Empty = 0,
    Sand,
    Water,
    Steel,
    Fire,
    Lava,
    GunPowder,
    Wood,
    BurntWood,
    Coal,
    Leaf,
    Vapor,
    Smoke,
    Snow,
    Size,
}

// TODO: Move this into a macro
impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            1 => Tile::Sand,
            2 => Tile::Water,
            3 => Tile::Steel,
            4 => Tile::Fire,
            5 => Tile::Lava,
            6 => Tile::GunPowder,
            7 => Tile::Wood,
            8 => Tile::BurntWood,
            9 => Tile::Coal,
            10 => Tile::Leaf,
            11 => Tile::Vapor,
            12 => Tile::Smoke,
            13 => Tile::Snow,
            _ => Tile::Empty,
        }
    }
}

const TILES_VERTEX_SHADER: &str = r#"
    #version 130
    in vec3 position;
    in vec2 texcoord;
    out vec2 uv;
    uniform mat4 Model;
    uniform mat4 Projection;

    void main(){
        vec4 res = Projection * Model * vec4(position, 1);
        uv = texcoord;
        gl_Position = res;
    }
"#;

const TILES_FRAGMENT_SHADER: &str = r#"
  #version 130

  #define ATLAS_SIZE 16u

  in vec2 uv;

  uniform ivec2 atlas_textures_size;
  uniform vec2 grid_size;
  uniform sampler2D atlas_texture;
  uniform sampler2D tiles;
  uniform sampler2D background;

  out vec4 fragColor;

  vec2 pixelate(vec2 uv, vec2 tex_size){
    return floor(uv * tex_size) / tex_size;
  }

  uint unpack_tile(vec2 uv){
    ivec2 grid_pos = ivec2(grid_size * uv);
    uint grid_idx = uint(grid_pos.y * grid_size.x + grid_pos.x);
    grid_pos.x /= 4;
    vec4 data = texelFetch(tiles, grid_pos, 0);
    return uint(floor(data[grid_idx%4u] * 255.0));
  }

  void main(){
    vec4 out_color;
    vec4 background = texture2D(background, uv);

    uint tile = unpack_tile(uv);
    ivec2 atlas_coords = atlas_textures_size * ivec2(tile % ATLAS_SIZE, tile / ATLAS_SIZE);
    ivec2 offset = ivec2(grid_size * uv);
    offset.x = offset.x % atlas_textures_size.x;
    offset.y = offset.y % atlas_textures_size.y;
    out_color = texelFetch(atlas_texture, atlas_coords + offset, 0);
    out_color = mix(out_color, background, background.a - out_color.a);
    fragColor = out_color;
  }
"#;

#[derive(Debug, Clone, Eq, PartialEq)]
struct DirtyRect {
    x: i16,
    y: i16,
    x2: i16,
    y2: i16,
}

impl DirtyRect {
    fn mark_dirty(&mut self, pos: (i16, i16)) {
        if self.is_empty() {
            self.x = pos.0;
            self.x2 = pos.0 + 1;
            self.y = pos.1;
            self.y2 = pos.1 + 1;
            return;
        }

        if pos.0 < self.x {
            self.x = pos.0;
        } else if pos.0 >= self.x2 {
            self.x2 = pos.0 + 1;
        }

        if pos.1 < self.y {
            self.y = pos.1;
        } else if pos.1 >= self.y2 {
            self.y2 = pos.1 + 1;
        }
    }

    fn is_empty(&self) -> bool {
        *self == DirtyRect::default()
    }
}

impl Default for DirtyRect {
    fn default() -> Self {
        Self {
            x: i16::MAX,
            y: i16::MAX,
            x2: i16::MIN,
            y2: i16::MIN,
        }
    }
}

pub struct RenderData {
    world_material: Option<Material>,
    tiles_atlas: Texture2D,
    tiles: Texture2D,
    background: Texture2D,
}

impl Default for RenderData {
    fn default() -> Self {
        Self {
            tiles: Texture2D::empty(),
            tiles_atlas: Texture2D::empty(),
            world_material: None,
            background: Texture2D::empty(),
        }
    }
}

pub struct Slot {
    id: u64,
    item: Option<Tile>,
}

impl Slot {
    fn new(id: u64) -> Slot {
        Slot { id, item: None }
    }

    fn new_with(id: u64, item: Tile) -> Slot {
        Slot {
            id,
            item: Some(item),
        }
    }
}

enum FittingCommand {
    Unfit { target_slot: u64 },
    Fit { target_slot: u64, item: Tile },
    Refit { target_slot: u64, origin_slot: u64 },
}

struct UIData {
    hot_bar: [(&'static str, Slot); 10],
    tile_palette: Vec<Tile>,
    palette_visible: bool,
    item_dragging: bool,
    selected: usize,
    fit_command: Option<FittingCommand>,
    tiles_atlas: Option<Image>,
    tile_textures: Vec<Texture2D>,
}

impl UIData {
    pub fn new() -> UIData {
        UIData {
            hot_bar: {
                [
                    ("1", Slot::new_with(hash!(), Tile::Sand)),
                    ("2", Slot::new_with(hash!(), Tile::Water)),
                    ("3", Slot::new_with(hash!(), Tile::Fire)),
                    ("4", Slot::new_with(hash!(), Tile::GunPowder)),
                    ("5", Slot::new(hash!())),
                    ("6", Slot::new(hash!())),
                    ("7", Slot::new(hash!())),
                    ("8", Slot::new(hash!())),
                    ("9", Slot::new(hash!())),
                    ("0", Slot::new(hash!())),
                ]
            },
            tile_palette: {
                let mut vec = Vec::<Tile>::with_capacity(Tile::Size as usize - 1);
                for i in 1..Tile::Size as u8 {
                    vec.push(i.into());
                }
                vec
            },
            palette_visible: true,
            item_dragging: false,
            selected: 0,
            fit_command: None,
            tiles_atlas: None,
            tile_textures: vec![],
        }
    }

    fn init_textures(&mut self) {
        if let Some(img) = &self.tiles_atlas {
            for i in 1..Tile::Size as i32 {
                self.tile_textures
                    .push(Texture2D::from_image(&img.sub_image(Rect {
                        x: (i % 16) as f32 * ATLAS_TEXTURES_SIZE as f32,
                        y: (i / 16) as f32 * ATLAS_TEXTURES_SIZE as f32,
                        w: ATLAS_TEXTURES_SIZE as f32,
                        h: ATLAS_TEXTURES_SIZE as f32,
                    })));
            }
        }
    }

    fn get_selected_tile(&self) -> Option<Tile> {
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
                .highlight(*item_dragging || self.selected == i)
                .ui(ui, |ui| {
                    if let Some(ref item) = slot.item {
                        ui.texture(
                            self.tile_textures[*item as usize - 1].clone(),
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
                    *item_dragging = false;
                }
                Drag::Dropped(_, None) if slot.item.is_some() => {
                    *fit_command = Some(FittingCommand::Unfit {
                        target_slot: (slot.id),
                    });
                    *item_dragging = false;
                }
                Drag::Dropped(_, _) => unreachable!(),
                Drag::Dragging(pos, id) => {
                    debug!("slots pos: {:?}, id: {:?}", pos, id);

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
                if self.tiles_atlas.is_some() {
                    ui.texture(
                        self.tile_textures[*item as usize - 1].clone(),
                        slot_size,
                        slot_size,
                    );
                }
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
                Drag::Dragging(pos, id) => {
                    debug!("tile palette pos: {:?}, id: {:?}", pos, id);

                    *item_dragging = true;
                }
                _ => {}
            }
        }
    }
    fn set_item(&mut self, id: u64, item: Option<Tile>) {
        for i in 0..self.hot_bar.len() {
            if self.hot_bar[i].1.id == id {
                self.hot_bar[i].1.item = item;
                break;
            }
        }
    }
}

impl Default for UIData {
    fn default() -> Self {
        Self::new()
    }
}
