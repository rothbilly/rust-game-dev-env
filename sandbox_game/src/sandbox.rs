use macroquack::core::app::AppRegistration; // To register it in the App registry.
use macroquack::core::game::{Game, GameLoop};
use macroquack::prelude::*;
use macroquad::miniquad::BlendState;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self},
};
use std::{cell::RefCell, rc::Rc};

use crate::{camera_controller::*, chunk::*, game_ui::*, paint_brush::*, particles::*, tile::*};

pub const ATLAS_TEXTURES_SIZE: i32 = 64;
pub const ATLAS_BYTES: &[u8] = include_bytes!("../assets/tile_atlas.png");
pub const BACKGROUND_BYTES: &[u8] = include_bytes!("../assets/background2.png");

// Noita keeps around 12 512x512 sized areas active at the same time
// This simulates a similar workload
//pub const WORLD_CHUNKS_WIDTH: i32 = 4 * 8 * 2;
//pub const WORLD_CHUNKS_HEIGHT: i32 = 3 * 8 * 2;

pub const WORLD_CHUNKS_WIDTH: i32 = 8;
pub const WORLD_CHUNKS_HEIGHT: i32 = 8;

// Noita's visible area

//pub const SCREEN_WIDTH: i32 = 480;
//pub const SCREEN_HEIGHT: i32 = 270;
pub const SCREEN_WIDTH: i32 = 256;
pub const SCREEN_HEIGHT: i32 = 256;

pub const ASPECT_RATIO: f32 = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;

pub const WORLD_BOUNDS: Rect = Rect {
    x: 0.0_f32,
    y: 0.0_f32,
    w: (Chunk::SIZE * WORLD_CHUNKS_WIDTH) as f32,
    h: (Chunk::SIZE * WORLD_CHUNKS_HEIGHT) as f32,
};

/*


  TODO: remove chunk queues and implement particle system

  TODO: rework gui

  TODO: save & load tiles to/from disk (png)
        TODO: add ui buttons to load and save files

  TODO: remove seams between chunks // has to do with chunk process order
        TODO: add multithreading

  TODO: add kenney patterns as brushes
        TODO: create new ui menu to select brushes

  TODO: implement Configurable keybinds
  TODO: implement Command/Action Design pattern to enable Undo/Redo while in pause

  TODO: add more tile types

**/

#[register_app("Sandbox")] // Custom Registration macro
#[auto_deref(base_game)]
pub struct Sandbox {
    base_game: Game,
    game_state: GameState,
    frame_stepping: bool,
    frame_counter: u64,
    last_mouse_pos: IVec2,
    paint_brush: PaintBrush,
    inventory: GameUI,
    camera: CameraController,
    chunks: Vec<Rc<RefCell<Chunk>>>,
    render_data: RenderData,
    water_particles: ParticleSystem,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum GameState {
    #[default]
    Paused,
    Running,
}

impl Default for Sandbox {
    fn default() -> Self {
        Self {
            base_game: Game::default(),
            game_state: GameState::Running,
            frame_stepping: false,
            last_mouse_pos: ivec2(0, 0),
            paint_brush: PaintBrush::default(),
            frame_counter: 0,
            inventory: GameUI::new(),
            camera: CameraController::default(),
            chunks: {
                let mut c: Vec<Rc<RefCell<Chunk>>> =
                    Vec::with_capacity((WORLD_CHUNKS_WIDTH * WORLD_CHUNKS_HEIGHT) as usize);
                for _ in 0..c.capacity() {
                    c.push(Rc::new(RefCell::new(Chunk::new().to_owned())));
                }
                c
            },
            render_data: RenderData::new(),
            water_particles: ParticleSystem::new(
                ParticleCreateInfo {
                    particle_id: Tile::Water,
                    mass: 1.0,
                    vel: vec2(0.0, 0.0),
                    acc: vec2(0.0, 0.0),
                    lifetime: (10000.0),
                },
                30000,
                || -> Vec2 { vec2(0.0, 9.81) },
            ),
        }
    }
}

impl GameLoop for Sandbox {
    fn _ready(&mut self) {
        {
            let positions: [IVec2; 8] = [
                ivec2(-1, -1),
                ivec2(0, -1),
                ivec2(1, -1),
                ivec2(-1, 0),
                ivec2(1, 0),
                ivec2(-1, 1),
                ivec2(0, 1),
                ivec2(1, 1),
            ];

            for y in 0..WORLD_CHUNKS_HEIGHT {
                for x in 0..WORLD_CHUNKS_WIDTH {
                    let mut neighbors = Vec::new();
                    let chunk_key = ivec2(x, y);
                    for pos in positions {
                        let n_key = chunk_key + pos;
                        if n_key.x >= 0
                            && n_key.x < WORLD_CHUNKS_WIDTH
                            && n_key.y >= 0
                            && n_key.y < WORLD_CHUNKS_HEIGHT
                        {
                            neighbors
                                .push((pos, self.chunks[chunk_idx_at_chunk_key(n_key)].clone()));
                        }
                    }
                    let chunk_idx = chunk_idx_at_chunk_key(chunk_key);
                    self.chunks[chunk_idx].borrow_mut().map_position = chunk_key;
                    self.chunks[chunk_idx].borrow_mut().init_maps(neighbors);
                }
            }
        }

        let mut world_center_x = (WORLD_CHUNKS_WIDTH / 2 * Chunk::SIZE) as f32;
        world_center_x += (if WORLD_CHUNKS_WIDTH % 2 == 0 {
            0
        } else {
            Chunk::SIZE / 2
        } as f32);

        let mut world_center_y = (WORLD_CHUNKS_HEIGHT / 2 * Chunk::SIZE) as f32;
        world_center_y += (if WORLD_CHUNKS_HEIGHT % 2 == 0 {
            0
        } else {
            Chunk::SIZE / 2
        } as f32);

        let half_screen_x = (SCREEN_WIDTH / 2) as f32;
        let half_screen_y = (SCREEN_HEIGHT / 2) as f32;
        self.camera.position = vec2(
            world_center_x - half_screen_x,
            world_center_y - half_screen_y,
        );

        let atlas_img = Image::from_file_with_format(ATLAS_BYTES, Some(ImageFormat::Png)).unwrap();
        self.inventory.tiles_atlas = Some(atlas_img.clone());
        self.inventory.init_textures();
        self.render_data.tiles_atlas = Texture2D::from_image(&atlas_img);
        self.render_data.tiles_atlas.set_filter(FilterMode::Nearest);

        //
        self.render_data.background =
            Texture2D::from_file_with_format(BACKGROUND_BYTES, Some(ImageFormat::Png));

        let mat = &self.render_data.camera_material;
        mat.set_uniform("atlas_textures_size", ATLAS_TEXTURES_SIZE);
        mat.set_uniform("grid_width", SCREEN_WIDTH);
        mat.set_uniform("debug_grid_size", Chunk::SIZE);
        mat.set_uniform("texture_scale", 128.0_f32);
        mat.set_texture("atlas_texture", self.render_data.tiles_atlas.clone());
        mat.set_texture("background", self.render_data.background.clone());
    }

    fn _process_input(&mut self) {
        let pressed_keys = get_keys_pressed();
        let mouse_wheel = mouse_wheel();
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

                KeyCode::LeftAlt => {
                    if self.render_data.debug_grid_size == 0 {
                        self.render_data.debug_grid_size = Chunk::SIZE;
                    } else {
                        self.render_data.debug_grid_size = 0;
                    }
                }
                KeyCode::LeftControl if is_mouse_button_down(MouseButton::Middle) => {
                    self.camera.reset_zoom();
                }
                KeyCode::N if self.game_state == GameState::Paused => {
                    self.frame_stepping = true;
                }
                _ => {}
            }
        }

        let held_keys = get_keys_down();

        let mut cam_dir = vec2(0.0, 0.0);
        for key in &held_keys {
            match key {
                KeyCode::W | KeyCode::Up => {
                    cam_dir.y = -1.0;
                }
                KeyCode::A | KeyCode::Left => {
                    cam_dir.x = -1.0;
                }
                KeyCode::S | KeyCode::Down => {
                    cam_dir.y = 1.0;
                }
                KeyCode::D | KeyCode::Right => {
                    cam_dir.x = 1.0;
                }
                KeyCode::LeftControl => {
                    self.camera.zoom_lerp += mouse_wheel.1 * 0.05;
                }
                _ => {}
            }
        }
        self.camera.direction = cam_dir;

        if let Some(t) = self.inventory.get_selected_tile() {
            self.paint_brush.set_tile(t);
        }

        self.process_ui();

        if !held_keys.contains(&KeyCode::LeftControl) {
            let amp = if held_keys.contains(&KeyCode::LeftShift) {
                5.0
            } else {
                1.0
            };
            self.paint_brush
                .set_size(self.paint_brush.size + (mouse_wheel.1.round() * amp) as i16);
        }

        if self.inventory.item_dragging {
            return;
        }

        let down_mouse_left = is_mouse_button_pressed(MouseButton::Left);
        let down_mouse_right = is_mouse_button_pressed(MouseButton::Right);
        let held_mouse_left = is_mouse_button_down(MouseButton::Left);
        let held_mouse_right = is_mouse_button_down(MouseButton::Right);

        if held_mouse_left
            && !down_mouse_left
            && let Some(_) = self.inventory.get_selected_tile()
        {
            self.paint_tiles(true);
            self.last_mouse_pos = to_ivec2(self.screen_to_world(mouse_position().into()));
        } else if held_mouse_right && !down_mouse_right {
            self.paint_brush.set_tile(Tile::Empty);
            self.paint_tiles(true);
            self.last_mouse_pos = to_ivec2(self.screen_to_world(mouse_position().into()));
        }

        let tile = if down_mouse_left && let Some(tile) = self.inventory.get_selected_tile() {
            Some(tile)
        } else if down_mouse_right {
            Some(Tile::Empty)
        } else {
            None
        };

        if down_mouse_left || down_mouse_right {
            self.last_mouse_pos = to_ivec2(self.screen_to_world(mouse_position().into()));
            self.paint_brush._drawing = true;
            if let Some(t) = tile {
                self.paint_brush.tile = t;
                self.paint_tiles(false);
            }
        }

        if is_mouse_button_released(MouseButton::Left)
            || is_mouse_button_released(MouseButton::Right)
        {
            self.paint_brush._drawing = false;
        }
    }

    fn _update(&mut self, _delta: f32) {
        self.camera.update(_delta);

        // update simulation
        match &self.game_state {
            GameState::Paused => {
                if self.frame_stepping {
                    self.game_state = GameState::Running;
                }
            }
            GameState::Running => {
                if self.frame_stepping {
                    self.game_state = GameState::Paused;
                    self.frame_stepping = false;
                }

                //self.simulate_chunks(ivec2(0, 0), 1);

                self.simulate_chunks(ivec2(0, 0), 2);
                self.simulate_chunks(ivec2(1, 1), 2);
                self.simulate_chunks(ivec2(1, 0), 2);
                self.simulate_chunks(ivec2(0, 1), 2);
            }
        }

        self.update_screen_tex();

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
        set_camera(self.camera.get());

        let cam_rect = self.camera.get_world_rect();
        draw_texture_ex(
            &self.render_data.background,
            cam_rect.x,
            cam_rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 {
                    x: cam_rect.w,
                    y: cam_rect.h,
                }),
                ..Default::default()
            },
        );

        let mat = &self.render_data.camera_material;
        mat.set_texture("tiles", self.render_data.camera_texture.clone());
        mat.set_uniform("debug_grid_size", self.render_data.debug_grid_size);
        mat.set_uniform("grid_width", SCREEN_WIDTH);

        gl_use_material(mat);

        draw_texture_ex(
            &self.render_data.camera_texture.clone(),
            cam_rect.x,
            cam_rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(cam_rect.w, cam_rect.h)),
                ..Default::default()
            },
        );

        unsafe {
            get_internal_gl().flush();
        }
        // draw brush
        if let Some(_) = self.inventory.get_selected_tile() {
            mat.set_texture("tiles", self.paint_brush.sampler_tex.clone());
            mat.set_uniform("debug_grid_size", 0_i32);
            mat.set_uniform(
                "grid_width",
                (self.paint_brush.sampler_img.width * 4) as i32,
            );
            let brush_size = self.paint_brush.max_size as f32;
            let mut mouse = self.screen_to_world(mouse_position().into());
            mouse.x -= brush_size / 2.0;
            mouse.y -= brush_size / 2.0;

            gl_use_material(mat);
            draw_rectangle(
                mouse.x.round(),
                mouse.y.round(),
                brush_size,
                brush_size,
                WHITE,
            );
        }

        gl_use_default_material();
    }

    fn _debug_draw(&self) {
        let mut chunk_idx: usize;
        for y in 0..WORLD_CHUNKS_HEIGHT {
            for x in 0..WORLD_CHUNKS_WIDTH {
                chunk_idx = chunk_idx_at_chunk_key(IVec2 { x, y });
                let chunk = &self.chunks[chunk_idx].borrow();
                let area = chunk.dirty_area;
                let world_pos = ((x * Chunk::SIZE), (y * Chunk::SIZE));
                if area.is_empty() {
                    continue;
                }
                let size = (area.x2 - area.x, area.y2 - area.y);
                draw_rectangle_lines(
                    (world_pos.0 + area.x) as f32,
                    (world_pos.1 + area.y) as f32,
                    size.0 as f32,
                    size.1 as f32,
                    2.0,
                    RED,
                );
            }
        }

        let cam_rect = self.camera.get_world_rect();
        draw_rectangle_lines(cam_rect.x, cam_rect.y, cam_rect.w, cam_rect.h, 1.0, ORANGE);

        set_default_camera();
        let mouse_pos = mouse_position();
        let mouse_world_pos = self.camera.screen_to_world(mouse_pos.into());
        draw_text(
            format!("Mouse: ({:.2}, {:.2})", mouse_pos.0, mouse_pos.1),
            64.0,
            64.0,
            24.0,
            WHITE,
        );
        draw_text(format!("{:?}", self.game_state), 64.0, 80.0, 24.0, WHITE);
        draw_text(
            format!("Mouse_world: {:.2}", mouse_world_pos),
            64.0,
            96.0,
            24.0,
            WHITE,
        );
        draw_text(
            format!("Camera_world: {:.2}", self.camera.position),
            64.0,
            112.0,
            24.0,
            WHITE,
        );
        draw_text(
            format!("Camera zoom: {:.2}", self.camera.zoom),
            64.0,
            128.0,
            24.0,
            WHITE,
        );
        draw_text(
            format!("Camera zoom lerp: {:.2}", self.camera.zoom_lerp),
            64.0,
            144.0,
            24.0,
            WHITE,
        );
    }
}

impl Sandbox {
    fn simulate_chunks(&mut self, start: IVec2, step: i32) {
        let mut x = start.x;
        let mut y = start.y;

        while y < WORLD_CHUNKS_HEIGHT {
            while x < WORLD_CHUNKS_WIDTH {
                self.simulate_chunk(x, y);
                x += step;
            }
            y += step;
            x = start.x;
        }
    }

    fn simulate_chunk(&mut self, x: i32, y: i32) {
        let mut chunk = self.chunks[chunk_idx_at_chunk_key(ivec2(x, y))].borrow_mut();

        if chunk.dirty_area.is_empty() {
            chunk.process_queues();
            return;
        }

        let area = chunk.dirty_area;
        chunk.dirty_area.reset();

        for y in area.y..area.y2 {
            for x in area.x..area.x2 {
                let pos = ivec2(x, y);
                if !in_chunk_bounds(pos) {
                    continue;
                }

                if chunk.process_tile(pos) {
                    chunk.notify_neighbors(pos);
                }
            }
        }
        chunk.process_queues();
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

    fn paint_tiles(&mut self, is_held: bool) {
        let mouse = self.screen_to_world(mouse_position().into());
        let mouse = vec2(mouse.x.floor(), mouse.y.floor());

        let chunks = &mut self.chunks;
        let c = &mut |p, t| {
            Sandbox::set_tile(chunks, p, t);
        };

        if !is_held {
            self.paint_brush
                .paint(ivec2(mouse.x as i32, mouse.y as i32), c);
            return;
        }

        self.paint_brush.paint_line_segment(
            self.last_mouse_pos,
            ivec2(mouse.x as i32, mouse.y as i32),
            c,
        );
        //self.paint_brush.paint(ivec2(grid_x, grid_y), c);
    }

    pub fn set_tile(chunks: &mut [Rc<RefCell<Chunk>>], pos: IVec2, tile: Tile) {
        let chunk_idx = chunk_idx_at_chunk_key(chunk_key_at_world_pos(pos));
        let mut chunk = chunks[chunk_idx].borrow_mut();
        let tile_idx = Chunk::pos_to_idx(world_to_chunk_local(pos));
        chunk.set_tile(tile_idx, tile);
        chunk.notify_neighbors(world_to_chunk_local(pos));
    }

    #[inline]
    fn screen_to_world(&self, pos: Vec2) -> Vec2 {
        self.camera.screen_to_world(pos)
    }

    fn update_screen_tex(&mut self) {
        let camera_rect = self.camera.get_world_rect();
        let start = ivec2(camera_rect.left() as i32, camera_rect.top() as i32);
        let tiles = &mut self.render_data.camera_tiles;
        // world position
        let mut tile_pos = start;

        for y in 0..SCREEN_HEIGHT {
            tile_pos.y = start.y + y;

            for x in 0..SCREEN_WIDTH {
                tile_pos.x = start.x + x;
                if !in_world_bounds(tile_pos) {
                    //set tile to empty
                    tiles[(y * SCREEN_WIDTH + x) as usize] = Tile::Empty as u8;
                    continue;
                }
                let chunk_key = chunk_key_at_world_pos(tile_pos);
                let chunk_tiles = &self.chunks[chunk_idx_at_chunk_key(chunk_key)]
                    .borrow()
                    .tiles;
                //copy tile
                tiles[(y * SCREEN_WIDTH + x) as usize] =
                    chunk_tiles[Chunk::pos_to_idx(world_to_chunk_local(tile_pos))]
            }
        }

        self.render_data.camera_texture.update_from_bytes(
            (SCREEN_WIDTH / 4) as u32,
            SCREEN_HEIGHT as u32,
            &tiles,
        );
    }
}

#[inline]
pub fn world_to_chunk_local(pos: IVec2) -> IVec2 {
    IVec2 {
        x: pos.x % Chunk::SIZE,
        y: pos.y % Chunk::SIZE,
    }
}

#[inline]
pub fn chunk_idx_at_chunk_key(pos: IVec2) -> usize {
    (pos.y * WORLD_CHUNKS_WIDTH + pos.x) as usize
}

#[inline]
pub fn chunk_key_at_world_pos(pos: IVec2) -> IVec2 {
    ivec2(pos.x / Chunk::SIZE, pos.y / Chunk::SIZE)
}

#[inline]
pub fn chunk_idx_at_world(world_pos: Vec2) -> usize {
    ((world_pos.y.floor() as i32 / Chunk::SIZE * WORLD_CHUNKS_WIDTH)
        + (world_pos.x.floor() as i32 / Chunk::SIZE)) as usize
}

#[inline]
pub fn in_world_bounds(pos: IVec2) -> bool {
    pos.x >= 0 && pos.x < WORLD_BOUNDS.w as i32 && pos.y >= 0 && pos.y < WORLD_BOUNDS.h as i32
}

#[inline]
pub fn in_chunk_bounds(pos: IVec2) -> bool {
    (pos.x >= 0) && (pos.x < Chunk::SIZE) && (pos.y >= 0) && (pos.y < Chunk::SIZE)
}

#[inline]
pub fn roll(chance: f32) -> bool {
    assert!(
        (0.0..=100.0).contains(&chance),
        "Chance for a roll must be equals or less than 100.0 and more than 0.0"
    );
    rand::gen_range(0.0, 100.0) <= chance
}

#[inline]
pub fn to_ivec2(pos: Vec2) -> IVec2 {
    ivec2(pos.x.round() as i32, pos.y.round() as i32)
}

pub fn get_display_rect() -> Rect {
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
const TILES_VERTEX_SHADER: &str = r#"
    #version 130
    in vec3 position;
    in vec2 texcoord;
    out vec2 uv;
    out vec2 uv_screen;
    uniform mat4 Model;
    uniform mat4 Projection;

    void main(){
        vec4 res = Projection * Model * vec4(position, 1);
        uv = texcoord;
        uv_screen = res.xy / 2.0 + vec2(0.5, 0.5);
        gl_Position = res;
    }
"#;

const TILES_FRAGMENT_SHADER: &str = r#"
  #version 130

  #define ATLAS_SIZE 16u

  in vec2 uv;
  in vec2 uv_screen;

  uniform int debug_grid_size;
  uniform int atlas_textures_size;
  uniform int grid_width;
  uniform float texture_scale;
  uniform sampler2D atlas_texture;
  uniform sampler2D tiles;
  uniform sampler2D background;

  out vec4 fragColor;

  vec2 pixelate(vec2 uv, vec2 tex_size){
    return floor(uv * tex_size) / tex_size;
  }

  uint unpack_tile(vec2 uv){
    ivec2 grid_pos = ivec2(grid_width * uv);
    uint grid_idx = uint(grid_pos.y * grid_width + grid_pos.x);
    grid_pos.x /= 4;
    vec4 data = texelFetch(tiles, grid_pos, 0);
    return uint(floor(data[grid_idx%4u] * 255.0));
  }

  void main(){
    vec4 out_color;
    uint tile = unpack_tile(uv);
    ivec2 atlas_coords = atlas_textures_size * ivec2(tile % ATLAS_SIZE, tile / ATLAS_SIZE);
    ivec2 offset = ivec2((uv * texture_scale));
    offset.x = offset.x % atlas_textures_size;
    offset.y = offset.y % atlas_textures_size;
    out_color = texelFetch(atlas_texture, atlas_coords + offset, 0);
    vec4 bg_color;
    if (debug_grid_size > 0){
        vec2 uv_bg;
        uv_bg = vec2(uv * debug_grid_size * 2.0);
        uv_bg.x = floor(uv_bg.x / debug_grid_size);
        uv_bg.y = floor(uv_bg.y / debug_grid_size);
        if ( int(uv_bg.x + uv_bg.y) % 2 == 0){
            bg_color = vec4(vec3(0.7), 0.5);
        }else{
            bg_color = vec4(vec3(0.2), 0.5);
        }
        out_color = mix(out_color, bg_color, bg_color.a - out_color.a);
    }


    fragColor = out_color;
  }
"#;

pub struct RenderData {
    //chunk_material: Material,
    //chunk_texture: Texture2D,
    camera_material: Material,
    camera_texture: Texture2D,
    camera_tiles: Vec<u8>,
    tiles_atlas: Texture2D,
    background: Texture2D,
    debug_grid_size: i32,
}

impl RenderData {
    fn new() -> Self {
        let mat = load_material(
            ShaderSource::Glsl {
                vertex: TILES_VERTEX_SHADER,
                fragment: TILES_FRAGMENT_SHADER,
            },
            MaterialParams {
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::Value(miniquad::BlendValue::SourceAlpha),
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    )),
                    ..Default::default()
                },
                uniforms: vec![
                    UniformDesc::new("texture_scale", UniformType::Float1),
                    UniformDesc::new("atlas_textures_size", UniformType::Int1),
                    UniformDesc::new("debug_grid_size", UniformType::Int1),
                    UniformDesc::new("grid_width", UniformType::Int1),
                ],
                textures: vec![
                    "tiles".to_owned(),
                    "atlas_texture".to_owned(),
                    "background".to_owned(),
                ],
            },
        )
        .expect("COULD NOT CREATE CAMERA MATERIAL");

        let tiles = vec![0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];
        let tex = Texture2D::from_rgba8(SCREEN_WIDTH as u16 / 4, SCREEN_HEIGHT as u16, &tiles);

        Self {
            camera_material: mat,
            camera_texture: tex,
            camera_tiles: tiles,
            tiles_atlas: Texture2D::empty(),
            background: Texture2D::empty(),
            debug_grid_size: 0_i32,
        }
    }
}
