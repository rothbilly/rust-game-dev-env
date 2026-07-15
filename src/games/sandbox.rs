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
const WORLD_WIDTH: i32 = 256;
const WORLD_HEIGHT: i32 = 240;
const ASPECT_RATIO: f32 = WORLD_WIDTH as f32 / WORLD_HEIGHT as f32;
const ATLAS_TEXTURES_SIZE: i32 = 1;

/*

    IMPORTANT: improve single thread performance before adding multithreading
    TODO: add spatial partitioning (chunks) and dirty rects
    TODO: load a f texture
    TODO: maybe add multithreading?? dont know how web build gonna react
    TODO: maybe add brush patterns
    TODO: fix shader uv scale?
    TODO: save & load tiles to/from disk (maybe bitmap? png? or just binary?)
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
}

impl Default for Sandbox {
    fn default() -> Self {
        let vec_size = (WORLD_WIDTH * WORLD_HEIGHT) as usize;
        Self {
            brush_size: 4,
            max_brush_size: 48,
            tiles: vec![EMPTY; vec_size],
            next_tiles: vec![EMPTY; vec_size],
            frame_counter: 0,
            inventory: UIData::new(),
            render_data: RenderData::default(),
            camera: Camera2D::default(),
            base: Default::default(),
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
                        UniformDesc::new("atlas_textures_size", UniformType::Float2),
                        UniformDesc::new("grid_size", UniformType::Float2),
                    ],
                    textures: vec!["atlas_texture".to_owned(), "tiles".to_owned()],
                    ..Default::default()
                },
            )
            .unwrap(),
        );

        if let Some(mat) = &self.render_data.world_material {
            // Hardcoded image until I figure out how to load one from disk(¡¿Futures + threads??!!)

            let mut img = Image::gen_image_color(
                16 * ATLAS_TEXTURES_SIZE as u16,
                16 * ATLAS_TEXTURES_SIZE as u16,
                Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
            );

            img.set_pixel(STEEL as u32, 0, Color::from_hex(0x4a5462));
            img.set_pixel(SAND as u32, 0, Color::from_hex(0xfef3c0));
            img.set_pixel(WATER as u32, 0, Color::from_hex(0x249fde));
            img.set_pixel(SMOKE as u32, 0, Color::from_hex(0x141013));
            img.set_pixel(VAPOR as u32, 0, Color::from_hex(0xdae0ea));
            img.set_pixel(SNOW as u32, 0, Color::from_hex(0xFFFFFF));
            img.set_pixel(WOOD as u32, 0, Color::from_hex(0x71413b));
            img.set_pixel(BURNT_WOOD as u32, 0, Color::from_hex(0x221c1a));
            img.set_pixel(LEAF as u32, 0, Color::from_hex(0x14a02e));
            img.set_pixel(COAL as u32, 0, Color::from_hex(0x242234));
            img.set_pixel(FIRE as u32, 0, Color::from_hex(0xfa6a0a));
            img.set_pixel(GUN_POWDER as u32, 0, Color::from_hex(0x333941));
            img.set_pixel(LAVA as u32, 0, Color::from_hex(0xb4202a));

            self.inventory.tiles_atlas = Some(img.clone());
            self.inventory.init_textures();
            self.render_data.tiles_atlas = Texture2D::from_image(&img);
            self.render_data.tiles_atlas.set_filter(FilterMode::Nearest);
            self.render_data.tiles = Texture2D::from_image(&self.render_data.tiles_image);
            self.render_data.tiles.set_filter(FilterMode::Nearest);
            mat.set_uniform("atlas_textures_size", vec2(1.0, 1.0));
            mat.set_uniform("grid_size", vec2(WORLD_WIDTH as f32, WORLD_HEIGHT as f32));
            mat.set_texture("atlas_texture", self.render_data.tiles_atlas.clone());
            mat.set_texture("tiles", self.render_data.tiles.clone());
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
            if *key == KeyCode::Tab {
                self.inventory.palette_visible = !self.inventory.palette_visible;
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
            self.paint_tiles(EMPTY);
            return;
        }

        if let Some(tile_type) = self.inventory.get_selected_tile() {
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

        // I wish i could pack this further but macroquad shader io is very limited :/
        // Maybe should use raw miniquad??
        //
        if let Some(material) = &self.render_data.world_material {
            let size = self.tiles.len();
            let tiles = &self.tiles;
            let mut i: u32 = 0;
            let img = &mut self.render_data.tiles_image;

            while i < size as u32 {
                let (x, y) = (
                    (i / 4) % (WORLD_WIDTH as u32 / 4),
                    (i) / (WORLD_WIDTH as u32),
                );
                {
                    let i = i as usize;
                    img.set_pixel(
                        x,
                        y,
                        Color::from_rgba(tiles[i], tiles[i + 1], tiles[i + 2], tiles[i + 3]),
                    );
                }
                i += 4;
            }

            self.render_data.tiles.update(&self.render_data.tiles_image);
            material.set_texture("tiles", self.render_data.tiles.clone());
        }

        self.frame_counter = (self.frame_counter + 1) % 60;
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
    }

    fn _render(&self) {
        set_camera(&self.camera);
        if let Some(mat) = &self.render_data.world_material {
            gl_use_material(mat);
            draw_rectangle(0.0, 0.0, WORLD_WIDTH as f32, WORLD_WIDTH as f32, WHITE);
            gl_use_default_material();
        }
        //draw_texture(&self.world_texture, 0.0, 0.0, WHITE);

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
                let mut origin_item: Option<u8> = None;
                let mut target_item: Option<u8> = None;
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

    fn paint_tiles(&mut self, tile_type: u8) {
        let mouse = self.mouse_to_world();
        let mouse = vec2(mouse.x.floor(), mouse.y.floor());

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
                if (neighbors[FIRE as usize] > 0 || neighbors[LAVA as usize] > 0) && self.roll(0.6)
                {
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
                    || ((neighbors[FIRE as usize] >= 2 || neighbors[LAVA as usize] > 0)
                        && self.roll(0.1))
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

                if (neighbors[FIRE as usize] >= 2 || neighbors[LAVA as usize] > 0) && self.roll(0.6)
                {
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
                if neighbors[FIRE as usize] > 1
                    || neighbors[LAVA as usize] > 0
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
                if (neighbors[FIRE as usize] + neighbors[LAVA as usize]) > neighbors[WATER as usize]
                    && self.roll(0.2)
                {
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
                if (neighbors[FIRE as usize] + neighbors[LAVA as usize]) - neighbors[WATER as usize]
                    > 1
                {
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
            LEAF => {
                if neighbors[FIRE as usize] > 1 || neighbors[LAVA as usize] > 0 {
                    self.set_tile(self.pos_to_idx(pos), FIRE);
                    return true;
                }

                if (neighbors[LEAF as usize] + neighbors[WOOD as usize] > 0) && self.roll(0.99) {
                    return false;
                }

                self.roll(0.1) && (self.try_move_down_sides(pos) || self.try_move_sides(pos))
            }
            COAL => {
                if ((neighbors[FIRE as usize] + neighbors[LAVA as usize])
                    - neighbors[WATER as usize])
                    > 0
                    && self.roll(0.05)
                {
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
            LAVA => {
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
const LEAF: u8 = 9;
const COAL: u8 = 10;
const FIRE: u8 = 11;
const GUN_POWDER: u8 = 12;
const LAVA: u8 = 13;
const TILE_TYPES: usize = 14;

const TILES_VERTEX_SHADER: &'static str = r#"
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

const TILES_FRAGMENT_SHADER: &'static str = r#"
  #version 130

  #define ATLAS_SIZE 16u

  in vec2 uv;

  uniform vec2 atlas_textures_size;
  uniform vec2 grid_size;
  uniform sampler2D atlas_texture;
  uniform sampler2D tiles;

  out vec4 fragColor;

  vec2 pixelate(vec2 uv, vec2 tex_size){
    return floor(uv * tex_size) / tex_size;
  }

  uint unpack_tile(vec2 uv){
    vec2 tile_texel = 1.0/textureSize(tiles, 0);
    vec2 grid_pos = floor(uv * grid_size);
    uint grid_idx = uint(floor(grid_pos.y * grid_size.x + grid_pos.x));
    vec2 tile_pos = vec2(0.0);
    tile_pos.x = grid_idx/4u % uint(floor(grid_size.x / 4.0));
    tile_pos.y = floor(grid_idx / grid_size.x);
    vec4 data = texture2D(tiles, tile_pos * tile_texel);
    return uint(floor(data[grid_idx%4u] * 255.0));
  }

  void main(){
    vec4 out_color;
    uint tile = unpack_tile(uv);
    vec2 atlas_texel = 1.0/textureSize(atlas_texture, 0);
    vec2 atlas_coords = atlas_texel * atlas_textures_size * vec2(uint(tile % ATLAS_SIZE), uint(tile / ATLAS_SIZE));
    out_color = texture2D(atlas_texture, atlas_coords);
    fragColor = out_color;
  }

"#;

pub struct RenderData {
    world_material: Option<Material>,
    tiles_atlas: Texture2D,
    tiles_image: Image,
    tiles: Texture2D,
}

impl Default for RenderData {
    fn default() -> Self {
        Self {
            tiles_image: Image::gen_image_color(WORLD_WIDTH as u16 / 4, WORLD_HEIGHT as u16, BLACK),
            tiles: Texture2D::empty(),
            world_material: None,
            tiles_atlas: Texture2D::empty(),
        }
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

struct UIData {
    hot_bar: [(&'static str, Slot); 10],
    tile_palette: Vec<u8>,
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
                    ("1", Slot::new_with(hash!(), SAND)),
                    ("2", Slot::new_with(hash!(), WATER)),
                    ("3", Slot::new_with(hash!(), FIRE)),
                    ("4", Slot::new_with(hash!(), GUN_POWDER)),
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
            tiles_atlas: None,
            tile_textures: vec![],
        }
    }

    fn init_textures(&mut self) {
        if let Some(img) = &self.tiles_atlas {
            for i in 1..TILE_TYPES {
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
    fn set_item(&mut self, id: u64, item: Option<u8>) {
        for i in 0..self.hot_bar.len() {
            if self.hot_bar[i].1.id == id {
                self.hot_bar[i].1.item = item;
                break;
            }
        }
    }
}
