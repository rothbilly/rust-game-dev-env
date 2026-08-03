use macroquack::core::app::AppRegistration; // To register it in the App registry.
use macroquack::core::game::{Game, GameLoop};
use macroquack::prelude::*;
use macroquad::miniquad::BlendState;

use macroquad::prelude::*;
use macroquad::ui::{
    Drag, Ui, hash, root_ui,
    widgets::{self, Group},
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const ATLAS_TEXTURES_SIZE: i32 = 64;
const ATLAS_BYTES: &[u8] = include_bytes!("../../../assets/sprites/sandbox/tile_atlas.png");
const BACKGROUND_BYTES: &[u8] = include_bytes!("../../../assets/sprites/sandbox/background1.png");
const WORLD_CHUNKS: i32 = 8;
const WORLD_SIZE: i32 = Chunk::SIZE * WORLD_CHUNKS;
const SCREEN_WIDTH: i32 = 512;
const SCREEN_HEIGHT: i32 = 512;
const ASPECT_RATIO: f32 = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;
const WORLD_BOUNDS: Rect = Rect {
    x: 0.0_f32,
    y: 0.0_f32,
    w: (Chunk::SIZE * WORLD_CHUNKS) as f32,
    h: (Chunk::SIZE * WORLD_CHUNKS) as f32,
};

/*
   TODO: remove chunk queues and implement particle system
   TODO: remove seams between chunks // has to do with chunk process order
   TODO: re implement tile behaviours
   TODO: save & load tiles to/from disk (maybe bitmap? png? or just binary?)
   TODO: implement Command/Action Design pattern to enable Undo/Redo while in pause
   IMPORTANT: improve single thread performance before adding multithreading
   TODO: maybe add multithreading??
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
    inventory: UIData,
    camera: CameraController,
    chunks: Vec<Rc<RefCell<Chunk>>>,
    render_data: RenderData,
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
            inventory: UIData::new(),
            camera: CameraController::default(),
            chunks: {
                let mut c: Vec<Rc<RefCell<Chunk>>> =
                    Vec::with_capacity((WORLD_CHUNKS * WORLD_CHUNKS) as usize);
                for _ in 0..c.capacity() {
                    c.push(Rc::new(RefCell::new(Chunk::new().to_owned())));
                }
                c
            },
            render_data: RenderData::new(),
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

            for y in 0..WORLD_CHUNKS {
                for x in 0..WORLD_CHUNKS {
                    let mut neighbors = Vec::new();
                    let chunk_key = ivec2(x, y);
                    for pos in positions {
                        let n_key = chunk_key + pos;
                        if n_key.x >= 0
                            && n_key.x < WORLD_CHUNKS
                            && n_key.y >= 0
                            && n_key.y < WORLD_CHUNKS
                        {
                            neighbors
                                .push((pos, self.chunks[chunk_idx_at_chunk_pos(n_key)].clone()));
                        }
                    }
                    let chunk_idx = chunk_idx_at_chunk_pos(chunk_key);
                    self.chunks[chunk_idx].borrow_mut().map_position = chunk_key;
                    self.chunks[chunk_idx].borrow_mut().init_maps(neighbors);
                }
            }
        }

        let world_center = (WORLD_CHUNKS / 2 * Chunk::SIZE) as f32;
        let half_screen = (SCREEN_WIDTH / 2) as f32;
        self.camera.position = vec2(world_center - half_screen, world_center - half_screen);

        let atlas_img = Image::from_file_with_format(ATLAS_BYTES, Some(ImageFormat::Png)).unwrap();
        self.inventory.tiles_atlas = Some(atlas_img.clone());
        self.inventory.init_textures();
        self.render_data.tiles_atlas = Texture2D::from_image(&atlas_img);
        self.render_data.tiles_atlas.set_filter(FilterMode::Nearest);

        //
        self.render_data.background =
            Texture2D::from_file_with_format(BACKGROUND_BYTES, Some(ImageFormat::Png));

        let mat = &self.render_data.chunk_material;
        mat.set_uniform("atlas_textures_size", ATLAS_TEXTURES_SIZE);
        mat.set_uniform("grid_size", Chunk::SIZE);
        mat.set_uniform("_debug", self.render_data._debug as i32);
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
                    self.render_data._debug = !self.render_data._debug;
                    self.render_data
                        .chunk_material
                        .set_uniform("_debug", self.render_data._debug as i32);
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
                    self.camera.zoom_dir = mouse_wheel.1;
                }
                _ => {}
            }
        }
        self.camera.direction = cam_dir;

        self.process_ui();

        if self.inventory.item_dragging {
            return;
        }

        if !held_keys.contains(&KeyCode::LeftControl) {
            let amp = if held_keys.contains(&KeyCode::LeftShift) {
                5.0
            } else {
                1.0
            };
            self.paint_brush.size += (mouse_wheel.1.round() * amp) as i16;
            self.paint_brush.size = self.paint_brush.size.clamp(1, self.paint_brush.max_size);
        }

        // TODO: refactor this

        let held_mouse_left = is_mouse_button_down(MouseButton::Left);
        let held_mouse_right = is_mouse_button_down(MouseButton::Right);
        let down_mouse_left = is_mouse_button_pressed(MouseButton::Left);
        let down_mouse_right = is_mouse_button_pressed(MouseButton::Right);

        if held_mouse_left
            && !down_mouse_left
            && let Some(tile) = self.inventory.get_selected_tile()
        {
            self.paint_brush.tile = tile;
            self.paint_tiles(true);
            self.last_mouse_pos = to_ivec2(self.screen_to_world(mouse_position().into()));
        } else if held_mouse_right && !down_mouse_right {
            self.paint_brush.tile = Tile::Empty;
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

        unsafe {
            let mut gl = get_internal_gl();
            let mut chunk_idx: usize;
            let mat = &self.render_data.chunk_material;
            let camera_rect = self.camera.get_world_rect();
            gl_use_material(mat);
            for y in 0..WORLD_CHUNKS {
                let cy = y * Chunk::SIZE;
                for x in 0..WORLD_CHUNKS {
                    chunk_idx = chunk_idx_at_chunk_pos(IVec2 { x, y });

                    if self.chunks[chunk_idx].borrow().is_empty()
                        || !self.chunks[chunk_idx]
                            .borrow()
                            .get_world_rect()
                            .overlaps(&camera_rect)
                    {
                        continue;
                    }

                    let cx = x * Chunk::SIZE;

                    let tiles = &self.chunks[chunk_idx].borrow().tiles;

                    self.render_data.chunk_texture.update_from_bytes(
                        (Chunk::SIZE / 4) as u32,
                        Chunk::SIZE as u32,
                        tiles,
                    );

                    mat.set_texture("tiles", self.render_data.chunk_texture.clone());
                    draw_rectangle(
                        cx as f32,
                        cy as f32,
                        Chunk::SIZE as f32,
                        Chunk::SIZE as f32,
                        WHITE,
                    );
                    gl.flush();
                }
            }
        }
        gl_use_default_material();

        let brush_size = self.paint_brush.size as f32;
        let mut mouse = self.screen_to_world(mouse_position().into());
        mouse.x -= brush_size / 2.0;
        mouse.y -= brush_size / 2.0;

        draw_rectangle_lines(
            mouse.x.round(),
            mouse.y.round(),
            brush_size,
            brush_size,
            2.0,
            WHITE,
        );
    }

    fn _debug_draw(&self) {
        let mut chunk_idx: usize;
        for y in 0..WORLD_CHUNKS {
            for x in 0..WORLD_CHUNKS {
                chunk_idx = chunk_idx_at_chunk_pos(IVec2 { x, y });
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

        set_default_camera();
        draw_rectangle_lines(cam_rect.x, cam_rect.y, cam_rect.w, cam_rect.y, 2.0, ORANGE);
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
    }
}

impl Sandbox {
    fn simulate_chunks(&mut self, start: IVec2, step: i32) {
        let mut x = start.x;
        let mut y = start.y;
        while y < WORLD_CHUNKS {
            while x < WORLD_CHUNKS {
                let mut chunk = self.chunks[chunk_idx_at_chunk_pos(ivec2(x, y))].borrow_mut();

                if chunk.dirty_area.is_empty() {
                    chunk.process_queues();
                    x += step;
                    continue;
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
                x += step;
            }
            y += step;
            x = start.x;
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
        let chunk_idx = chunk_idx_at_chunk_pos(chunk_pos_at_world(pos));
        let mut chunk = chunks[chunk_idx].borrow_mut();
        let tile_idx = chunk.pos_to_idx(world_to_chunk_local(pos));
        chunk.set_tile(tile_idx, tile);
        chunk.notify_neighbors(world_to_chunk_local(pos));
    }

    #[inline]
    fn screen_to_world(&self, pos: Vec2) -> Vec2 {
        self.camera.screen_to_world(pos)
    }
}

#[inline]
fn world_to_chunk_local(pos: IVec2) -> IVec2 {
    IVec2 {
        x: pos.x % Chunk::SIZE,
        y: pos.y % Chunk::SIZE,
    }
}

#[inline]
fn chunk_idx_at_chunk_pos(pos: IVec2) -> usize {
    (pos.y * WORLD_CHUNKS + pos.x) as usize
}

#[inline]
fn chunk_pos_at_world(pos: IVec2) -> IVec2 {
    ivec2(pos.x / Chunk::SIZE, pos.y / Chunk::SIZE)
}

#[inline]
fn chunk_idx_at_world(world_pos: Vec2) -> usize {
    ((world_pos.y.floor() as i32 / Chunk::SIZE * WORLD_CHUNKS)
        + (world_pos.x.floor() as i32 / Chunk::SIZE)) as usize
}

#[inline]
fn in_world_bounds(pos: IVec2) -> bool {
    pos.x >= 0 && pos.x < WORLD_SIZE && pos.y >= 0 && pos.y < WORLD_SIZE
}

#[inline]
fn in_chunk_bounds(pos: IVec2) -> bool {
    (pos.x >= 0) && (pos.x < Chunk::SIZE) && (pos.y >= 0) && (pos.y < Chunk::SIZE)
}

#[inline]
fn roll(chance: f32) -> bool {
    assert!(
        (0.0..=100.0).contains(&chance),
        "Chance for a roll must be equals or less than 100.0 and more than 0.0"
    );
    rand::gen_range(0.0, 100.0) <= chance
}

#[inline]
fn to_ivec2(pos: Vec2) -> IVec2 {
    ivec2(pos.x.round() as i32, pos.y.round() as i32)
}

fn get_display_rect() -> Rect {
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

#[derive(Clone, Debug)]
pub struct Chunk {
    // position inside sandbox's chunks collection
    map_position: IVec2,
    dirty_area: DirtyRect,
    neighbors: HashMap<IVec2, Rc<RefCell<Chunk>>>,
    tiles: Vec<u8>,

    // to check empty chunk
    tile_counter: i64,
    // moves inside the chunk are queued immediately
    move_queue: Vec<(usize, usize)>,
    // moves received from a neighboring chunk are performed first
    receiving_moves: Vec<((IVec2, usize), usize)>,

    // tmp buffer
    filled_moves: Vec<usize>,
}

impl Chunk {
    const SIZE: i32 = 64;
    const SIZE_SQR: i32 = Chunk::SIZE * Chunk::SIZE;

    fn new() -> Self {
        Chunk {
            tile_counter: 0,
            map_position: IVec2 { x: 0, y: 0 },
            tiles: vec![Tile::Empty as u8; (Chunk::SIZE * Chunk::SIZE) as usize],
            move_queue: Vec::with_capacity((Chunk::SIZE * Chunk::SIZE) as usize),
            dirty_area: DirtyRect::default(),
            neighbors: HashMap::with_capacity(8),
            // at most a full square of tiles can get into each chunk
            receiving_moves: Vec::with_capacity(Chunk::SIZE as usize * 4),
            filled_moves: Vec::with_capacity(Chunk::SIZE as usize * 4),
        }
    }

    fn init_maps(&mut self, neighbors: Vec<(IVec2, Rc<RefCell<Chunk>>)>) {
        for neighbor in neighbors {
            _ = self.neighbors.insert(neighbor.0, neighbor.1.clone());
        }
    }

    // Processing the queues takes about 10%-35% of the frame time, most of the time is spent sorting them
    // Should be better if we removed them and use a simple system similar to noita's
    // where we only update tiles that fall down from the bottom up, and tiles that go up get put
    // into a particle system
    fn process_queues(&mut self) {
        self.receiving_moves.sort_unstable_by_key(|a| a.1);
        let size = self.receiving_moves.len();
        let mut prev: usize = 0;
        for i in 0..size {
            if i + 1 == size || self.receiving_moves[i + 1].1 != self.receiving_moves[i].1 {
                let rng: usize = rand::gen_range(prev, i + 1);
                let ((chunk_key, src), dst) = self.receiving_moves[rng];
                if let Some(chunk) = self.neighbors.get(&chunk_key) {
                    let src_tile: Tile = chunk.borrow().tiles[src].into();
                    chunk.borrow_mut().set_tile(src, self.tiles[dst].into());
                    self.set_tile(dst, src_tile);
                    prev = i + 1;
                    self.filled_moves.push(dst);
                }
            }
        }

        // dont care about original order
        self.move_queue.sort_unstable_by_key(|a| a.0);

        let size = self.move_queue.len();
        prev = 0;
        for i in 0..size {
            if i + 1 == size || self.move_queue[i + 1].0 != self.move_queue[i].0 {
                let rng: usize = rand::gen_range(prev, i + 1);
                let (dst, src) = self.move_queue[rng];
                if self.filled_moves.contains(&dst) {
                    for k in prev..i + 1 {
                        self.dirty_area
                            .mark_dirty(self.idx_to_pos(self.move_queue[k].1));
                    }
                    prev = i + 1;
                    continue;
                }

                let src_tile: Tile = self.tiles[src].into();
                self.set_tile(src, self.tiles[dst].into());
                self.set_tile(dst, src_tile);
                prev = i + 1;
            }
        }

        self.receiving_moves.clear();
        self.move_queue.clear();
        self.filled_moves.clear();
    }

    // T_T
    fn notify_neighbors(&mut self, tile_pos: IVec2) {
        if tile_pos.x == 0 && tile_pos.y == 0 {
            if let Some(n_chunk) = self.neighbors.get(&ivec2(-1, -1)) {
                n_chunk
                    .borrow_mut()
                    .dirty_area
                    .mark_dirty(ivec2(Chunk::SIZE - 1, Chunk::SIZE - 1));
            }
            if let Some(n_chunk) = self.neighbors.get(&ivec2(-1, 0)) {
                n_chunk
                    .borrow_mut()
                    .dirty_area
                    .mark_dirty(ivec2(Chunk::SIZE - 1, 0));
            }
            if let Some(n_chunk) = self.neighbors.get(&ivec2(0, -1)) {
                n_chunk
                    .borrow_mut()
                    .dirty_area
                    .mark_dirty(ivec2(0, Chunk::SIZE - 1));
            }
        } else if tile_pos.x == (Chunk::SIZE - 1) && tile_pos.y == 0 {
            if let Some(n_chunk) = self.neighbors.get(&ivec2(1, -1)) {
                n_chunk
                    .borrow_mut()
                    .dirty_area
                    .mark_dirty(ivec2(0, Chunk::SIZE - 1));
            }
            if let Some(n_chunk) = self.neighbors.get(&ivec2(1, 0)) {
                n_chunk.borrow_mut().dirty_area.mark_dirty(ivec2(0, 0));
            }
            if let Some(n_chunk) = self.neighbors.get(&ivec2(0, -1)) {
                n_chunk
                    .borrow_mut()
                    .dirty_area
                    .mark_dirty(ivec2(Chunk::SIZE - 1, Chunk::SIZE - 1));
            }
        } else if tile_pos.x == 0 && tile_pos.y == (Chunk::SIZE - 1) {
            if let Some(n_chunk) = self.neighbors.get(&ivec2(-1, 1)) {
                n_chunk
                    .borrow_mut()
                    .dirty_area
                    .mark_dirty(ivec2(Chunk::SIZE - 1, 0));
            }
            if let Some(n_chunk) = self.neighbors.get(&ivec2(-1, 0)) {
                n_chunk
                    .borrow_mut()
                    .dirty_area
                    .mark_dirty(ivec2(Chunk::SIZE - 1, Chunk::SIZE - 1));
            }
            if let Some(n_chunk) = self.neighbors.get(&ivec2(0, 1)) {
                n_chunk.borrow_mut().dirty_area.mark_dirty(ivec2(0, 0));
            }
        } else if tile_pos.x == (Chunk::SIZE - 1) && tile_pos.y == (Chunk::SIZE - 1) {
            if let Some(n_chunk) = self.neighbors.get(&ivec2(1, 1)) {
                n_chunk.borrow_mut().dirty_area.mark_dirty(ivec2(0, 0));
            }
            if let Some(n_chunk) = self.neighbors.get(&ivec2(1, 0)) {
                n_chunk
                    .borrow_mut()
                    .dirty_area
                    .mark_dirty(ivec2(0, Chunk::SIZE - 1));
            }
            if let Some(n_chunk) = self.neighbors.get(&ivec2(0, 1)) {
                n_chunk
                    .borrow_mut()
                    .dirty_area
                    .mark_dirty(ivec2(Chunk::SIZE - 1, 0));
            }
        } else if tile_pos.x == 0
            && let Some(n_chunk) = self.neighbors.get(&ivec2(-1, 0))
        {
            n_chunk
                .borrow_mut()
                .dirty_area
                .mark_dirty(ivec2(Chunk::SIZE - 1, tile_pos.y));
        } else if tile_pos.x == (Chunk::SIZE - 1)
            && let Some(n_chunk) = self.neighbors.get(&ivec2(1, 0))
        {
            n_chunk
                .borrow_mut()
                .dirty_area
                .mark_dirty(ivec2(0, tile_pos.y));
        } else if tile_pos.y == 0
            && let Some(n_chunk) = self.neighbors.get(&ivec2(0, -1))
        {
            n_chunk
                .borrow_mut()
                .dirty_area
                .mark_dirty(ivec2(tile_pos.x, Chunk::SIZE - 1));
        } else if tile_pos.y == (Chunk::SIZE - 1)
            && let Some(n_chunk) = self.neighbors.get(&ivec2(0, 1))
        {
            n_chunk
                .borrow_mut()
                .dirty_area
                .mark_dirty(ivec2(tile_pos.x, 0));
        }
    }

    // Counting the neighbors can take around 20% of the frame time, maybe try counting
    // only when needed? or take a different approach?
    fn process_tile(&mut self, pos: IVec2) -> bool {
        let tile: Tile = self.tiles[self.pos_to_idx(pos)].into();

        let will_move = (|| match tile {
            Tile::Sand => self.try_move_down(&tile, pos) || self.try_move_down_sides(&tile, pos),
            Tile::Water => {
                let neighbors = self.get_neighbors_per_type(pos);

                if neighbors[Tile::Fire as usize] >= 3 || neighbors[Tile::Lava as usize] > 0 {
                    self.set_tile(self.pos_to_idx(pos), Tile::Steam);
                }

                (roll(22.5) && self.try_move_down_sides(&tile, pos))
                    || self.try_move_down(&tile, pos)
                    || self.try_move_down_sides(&tile, pos)
                    || self.try_move_sides(&tile, pos)
            }
            Tile::Fire => {
                if roll(33.0) {
                    self.set_tile(self.pos_to_idx(pos), Tile::Empty);
                    return true;
                }

                let positions = [
                    ivec2(-1, -1),
                    ivec2(-1, -1),
                    ivec2(-1, -1),
                    ivec2(0, -1),
                    ivec2(0, -1),
                    ivec2(0, -1),
                    ivec2(1, -1),
                    ivec2(1, -1),
                    ivec2(1, -1),
                    ivec2(-1, 0),
                    ivec2(1, 0),
                    ivec2(-1, 1),
                    ivec2(0, 1),
                    ivec2(1, 1),
                ];
                let rng = rand::gen_range(0_usize, positions.len());

                self.try_move(&tile, pos, positions[rng])
            }
            Tile::GunPowder => {
                let neighbors = self.get_neighbors_per_type(pos);

                if (neighbors[Tile::Fire as usize] >= 1 || neighbors[Tile::Lava as usize] >= 1)
                    && roll(33.0)
                {
                    self.set_tile(self.pos_to_idx(pos), Tile::Fire);
                    return false;
                }

                self.try_move_down(&tile, pos) || self.try_move_down_sides(&tile, pos)
            }
            Tile::Lava => {
                roll(25.0)
                    && (self.try_move_down(&tile, pos) || self.try_move_down_sides(&tile, pos))
            }
            Tile::Steam => {
                let neighbors = self.get_neighbors_per_type(pos);

                if neighbors[Tile::Steam as usize] >= 8 && roll(0.005) {
                    self.set_tile(self.pos_to_idx(pos), Tile::Water);
                    return true;
                }

                (roll(33.0) && self.try_move_up_sides(&tile, pos))
                    || self.try_move_up(&tile, pos)
                    || self.try_move_up_sides(&tile, pos)
                    || self.try_move_sides(&tile, pos)
            }
            Tile::Smoke => {
                if roll(0.33) {
                    self.set_tile(self.pos_to_idx(pos), Tile::Empty);
                    return true;
                }

                let neighbors = self.get_neighbors_per_type(pos);
                if (neighbors[Tile::Fire as usize] > 0 || neighbors[Tile::Lava as usize] > 0)
                    && roll(87.0)
                {
                    self.set_tile(self.pos_to_idx(pos), Tile::Fire);
                    return true;
                }

                (roll(33.0) && self.try_move_up_sides(&tile, pos))
                    || self.try_move_up(&tile, pos)
                    || self.try_move_up_sides(&tile, pos)
                    || self.try_move_sides(&tile, pos)
            }
            Tile::Wood => {
                let neighbors = self.get_neighbors_per_type(pos);
                if (neighbors[Tile::Fire as usize] >= 2 || neighbors[Tile::Lava as usize] >= 1)
                    && roll(66.0)
                {
                    self.set_tile(self.pos_to_idx(pos), Tile::BurntWood);
                }
                false
            }
            Tile::BurntWood => {
                let neighbors = self.get_neighbors_per_type(pos);
                if (neighbors[Tile::Fire as usize] > 0 || neighbors[Tile::Lava as usize] > 0)
                    && roll(87.0)
                {
                    self.set_tile(self.pos_to_idx(pos), Tile::Coal);
                }
                self.try_move_down(&tile, pos)
                    || (roll(25.0) && self.try_move_down_sides(&tile, pos))
            }
            Tile::Coal => {
                let neighbors = self.get_neighbors_per_type(pos);
                if (neighbors[Tile::Fire as usize] > 0 || neighbors[Tile::Lava as usize] > 0)
                    && roll(76.0)
                {
                    self.set_tile(self.pos_to_idx(pos), Tile::Fire);
                }
                self.try_move_down(&tile, pos)
                    || (roll(25.0) && self.try_move_down_sides(&tile, pos))
            }
            Tile::Leaf => {
                let neighbors = self.get_neighbors_per_type(pos);
                if (neighbors[Tile::Fire as usize] > 0 || neighbors[Tile::Lava as usize] > 0)
                    && roll(78.0)
                {
                    self.set_tile(self.pos_to_idx(pos), Tile::Fire);
                    return true;
                }

                if (neighbors[Tile::Leaf as usize] + neighbors[Tile::Wood as usize] > 0)
                    && roll(99.9)
                {
                    return false;
                }

                (roll(66.0) && self.try_move_sides(&tile, pos))
                    || (roll(33.0) && self.try_move_down_sides(&tile, pos))
                    || self.try_move_down(&tile, pos)
                    || self.try_move_down_sides(&tile, pos)
                    || self.try_move_sides(&tile, pos)
            }
            Tile::Snow => {
                let neighbors = self.get_neighbors_per_type(pos);
                if ((neighbors[Tile::Water as usize] >= 4) && roll(33.0))
                    || (neighbors[Tile::Fire as usize] > 0 && roll(33.0))
                    || (neighbors[Tile::Lava as usize] > 0 && roll(66.0))
                {
                    self.set_tile(self.pos_to_idx(pos), Tile::Water);
                    return true;
                }

                roll(33.0)
                    && ((roll(66.0) && self.try_move_down_sides(&tile, pos))
                        || (roll(33.0) && self.try_move_sides(&tile, pos))
                        || self.try_move_down(&tile, pos)
                        || self.try_move_down_sides(&tile, pos)
                        || self.try_move_sides(&tile, pos))
            }
            _ => false,
        })();

        if !will_move
            && matches!(
                tile,
                Tile::Fire | Tile::Lava | Tile::Steam | Tile::Smoke | Tile::Leaf | Tile::Snow
            )
        {
            self.dirty_area.mark_dirty(pos);
        }

        will_move
    }

    fn set_tile(&mut self, idx: usize, tile: Tile) {
        if (tile == Tile::Empty && self.tiles[idx] != Tile::Empty as u8)
            || (tile != Tile::Empty && self.tiles[idx] == Tile::Empty as u8)
        {
            self.tile_counter += if tile != Tile::Empty { 1 } else { -1 };
        }
        self.tiles[idx] = tile as u8;

        self.dirty_area.mark_dirty(ivec2(
            (idx as i32) % Chunk::SIZE,
            (idx as i32) / Chunk::SIZE,
        ));
    }

    fn try_move(&mut self, tile: &Tile, pos: IVec2, dir: IVec2) -> bool {
        let from_idx = self.pos_to_idx(pos);
        let to = pos + dir;
        let to_tile: Tile;
        if !in_chunk_bounds(to) {
            if !in_world_bounds(self.local_to_world_pos(to)) {
                return false;
            }

            let (n_key, to_idx) = self.calculate_neighbor_info(pos, dir);
            if let Some(chunk) = self.neighbors.get(&n_key) {
                to_tile = chunk.borrow().tiles[to_idx].into();

                if !tile.can_swap(&to_tile, dir) {
                    return false;
                }

                chunk
                    .borrow_mut()
                    .receiving_moves
                    .push(((-n_key, from_idx), to_idx));
                return true;
            }
            return false;
        }

        let to_idx: usize = self.pos_to_idx(to);
        to_tile = self.tiles[to_idx].into();

        if !tile.can_swap(&to_tile, dir) {
            return false;
        }

        self.move_queue.push((to_idx, from_idx));

        true
    }

    #[inline]
    fn try_move_down(&mut self, tile: &Tile, pos: IVec2) -> bool {
        self.try_move(tile, pos, ivec2(0, 1))
    }

    #[inline]
    fn try_move_up(&mut self, tile: &Tile, pos: IVec2) -> bool {
        self.try_move(tile, pos, ivec2(0, -1))
    }

    fn try_move_down_sides(&mut self, tile: &Tile, pos: IVec2) -> bool {
        let options = [ivec2(1, 1), ivec2(-1, 1)];
        let idx = rand::gen_range(0, 2);
        if !self.try_move(tile, pos, options[idx as usize]) {
            return self.try_move(tile, pos, options[(idx - 1_i32).unsigned_abs() as usize]);
        }

        true
    }

    fn try_move_up_sides(&mut self, tile: &Tile, pos: IVec2) -> bool {
        let options = [ivec2(1, -1), ivec2(-1, -1)];
        let idx = rand::gen_range(0, 2);
        if !self.try_move(tile, pos, options[idx as usize]) {
            return self.try_move(tile, pos, options[(idx - 1_i32).unsigned_abs() as usize]);
        }
        true
    }

    fn try_move_sides(&mut self, tile: &Tile, pos: IVec2) -> bool {
        let options = [ivec2(1, 0), ivec2(-1, 0)];
        let idx = rand::gen_range(0, 2);
        if !self.try_move(tile, pos, options[idx as usize]) {
            return self.try_move(tile, pos, options[(idx - 1_i32).unsigned_abs() as usize]);
        }
        true
    }

    // the index is the type
    // probably bad idea in the long run, but works fine for now
    fn get_neighbors_per_type(&self, from: IVec2) -> [i8; Tile::Size as usize] {
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

        let mut neighbors = [0_i8; Tile::Size as usize];

        let mut tile: u8;
        for pos in &positions {
            let n_pos = from + *pos;

            if !in_chunk_bounds(n_pos) {
                if !in_world_bounds(self.local_to_world_pos(n_pos)) {
                    continue;
                }
                let (n_key, idx) = self.calculate_neighbor_info(from, *pos);
                if let Some(chunk) = self.neighbors.get(&n_key) {
                    tile = chunk.borrow().tiles[idx];
                    neighbors[tile as usize] += 1;
                }
                continue;
            }
            tile = self.tiles[self.pos_to_idx(n_pos)];
            neighbors[tile as usize] += 1;
        }

        neighbors
    }

    #[inline]
    fn pos_to_idx(&self, pos: IVec2) -> usize {
        (pos.y * Chunk::SIZE + pos.x) as usize
    }

    #[inline]
    fn idx_to_pos(&self, idx: usize) -> IVec2 {
        ivec2(idx as i32 % Chunk::SIZE, (idx as i32) * (1 / Chunk::SIZE))
    }

    fn local_to_world_pos(&self, pos: IVec2) -> IVec2 {
        self.map_position * Chunk::SIZE + pos
    }

    fn get_world_rect(&self) -> Rect {
        Rect {
            x: (self.map_position.x * Chunk::SIZE) as f32,
            y: (self.map_position.y * Chunk::SIZE) as f32,
            w: Chunk::SIZE as f32,
            h: Chunk::SIZE as f32,
        }
    }

    fn is_empty(&self) -> bool {
        self.tile_counter == 0
    }

    // T_T
    fn calculate_neighbor_info(&self, pos: IVec2, dir: IVec2) -> (IVec2, usize) {
        let n_key: IVec2;
        let idx: usize;

        // top left
        if pos.x == 0 && pos.y == 0 {
            if dir.x == -1 && dir.y == -1 {
                n_key = dir;
                idx = ((Chunk::SIZE_SQR) - 1) as usize;
            } else if dir.y == -1 && dir.x == 1 {
                n_key = ivec2(0, -1);
                idx = (Chunk::SIZE * (Chunk::SIZE - 1) + 1) as usize;
            } else if dir.x == -1 && dir.y == 1 {
                n_key = ivec2(-1, 0);
                idx = 2 * (Chunk::SIZE - 1) as usize;
            } else if dir.y == -1 {
                n_key = ivec2(0, -1);
                idx = (Chunk::SIZE * (Chunk::SIZE - 1)) as usize;
            } else if dir.x == -1 {
                n_key = ivec2(-1, 0);
                idx = (Chunk::SIZE - 1) as usize;
            } else {
                panic!("Imposible as calculate neighbor info top left");
            }
        }
        // top right
        else if pos.x == Chunk::SIZE - 1 && pos.y == 0 {
            if dir.x == 1 && dir.y == -1 {
                n_key = dir;
                idx = (Chunk::SIZE * (Chunk::SIZE - 1)) as usize;
            } else if dir.y == -1 && dir.x == -1 {
                n_key = ivec2(0, -1);
                idx = ((Chunk::SIZE * Chunk::SIZE) - 1) as usize;
            } else if dir.y == -1 {
                n_key = ivec2(0, -1);
                idx = ((Chunk::SIZE_SQR) - 1) as usize;
            } else if dir.y == 1 && dir.x == 1 {
                n_key = ivec2(1, 0);
                idx = Chunk::SIZE as usize;
            } else if dir.x == 1 {
                n_key = ivec2(1, 0);
                idx = 0_usize;
            } else {
                panic!("Imposible as calculate neighbor info top right");
            }
        }
        // bottom left
        else if pos.x == 0 && pos.y == Chunk::SIZE - 1 {
            if dir.x == -1 && dir.y == 1 {
                n_key = dir;
                idx = (Chunk::SIZE - 1) as usize;
            } else if dir.y == 1 && dir.x == 1 {
                n_key = ivec2(0, 1);
                idx = 1_usize;
            } else if dir.x == -1 && dir.y == -1 {
                n_key = ivec2(-1, 0);
                idx = (Chunk::SIZE * (Chunk::SIZE - 1) - 1) as usize;
            } else if dir.y == 1 {
                n_key = ivec2(0, 1);
                idx = 0_usize;
            } else if dir.x == -1 {
                n_key = ivec2(-1, 0);
                idx = (Chunk::SIZE_SQR - 1) as usize;
            } else {
                panic!("Imposible as calculate neighbor info bottom left");
            }
        }
        // bottom right
        else if pos.x == (Chunk::SIZE - 1) && pos.y == (Chunk::SIZE - 1) {
            if dir.x == 1 && dir.y == 1 {
                n_key = dir;
                idx = 0_usize;
            } else if dir.x == 1 && dir.y == -1 {
                n_key = ivec2(1, 0);
                idx = (Chunk::SIZE * (Chunk::SIZE - 2)) as usize;
            } else if dir.y == 1 && dir.x == -1 {
                n_key = ivec2(0, 1);
                idx = (Chunk::SIZE - 2) as usize;
            } else if dir.y == 1 {
                n_key = ivec2(0, 1);
                idx = (Chunk::SIZE - 1) as usize;
            } else if dir.x == 1 {
                n_key = ivec2(1, 0);
                idx = (Chunk::SIZE * (Chunk::SIZE - 1)) as usize;
            } else {
                panic!("Imposible as calculate neighbor info bottom right");
            }
        }
        // middle left
        else if pos.x == 0 {
            n_key = ivec2(-1, 0);
            idx = self.pos_to_idx(ivec2(Chunk::SIZE - 1, pos.y + dir.y));
        }
        // middle right
        else if pos.x == Chunk::SIZE - 1 {
            n_key = ivec2(1, 0);
            idx = self.pos_to_idx(ivec2(0, pos.y + dir.y));
        }
        // middle top
        else if pos.y == 0 {
            n_key = ivec2(0, -1);
            idx = self.pos_to_idx(ivec2(pos.x + dir.x, Chunk::SIZE - 1));
        }
        // middle bottom
        else if pos.y == Chunk::SIZE - 1 {
            n_key = ivec2(0, 1);
            idx = self.pos_to_idx(ivec2(pos.x + dir.x, 0));
        }
        // MONKA
        else {
            panic!("Imposible at calculate_neighbor_info");
        }

        (n_key, idx)
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Tile {
    Empty,
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
    Steam,
    Smoke,
    Snow,
    Size,
}

impl Tile {
    fn get_density(&self) -> f32 {
        match self {
            Tile::Empty => 0.0,
            Tile::Steam => 0.6,
            Tile::Smoke => 0.85,
            Tile::Leaf => 0.95,
            Tile::Water => 1.0,
            Tile::Sand => 1.6,
            Tile::GunPowder => 1.2,
            Tile::Wood => 0.77,

            _ => 1.0,
        }
    }

    fn is_liquid_or_gas(&self) -> bool {
        matches!(self, Self::Water | Self::Fire | Self::Steam | Self::Smoke)
    }

    fn can_swap(&self, other: &Tile, dir: IVec2) -> bool {
        if self.is_liquid_or_gas() || other.is_liquid_or_gas() {
            (dir.y == 1 && self.get_density() > other.get_density())
                || (dir.y == -1 && self.get_density() < other.get_density())
                || *other == Tile::Empty
        } else {
            *other == Tile::Empty
        }
    }
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
            11 => Tile::Steam,
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

  uniform int _debug;
  uniform int atlas_textures_size;
  uniform int grid_size;
  uniform float texture_scale;
  uniform sampler2D atlas_texture;
  uniform sampler2D tiles;
  uniform sampler2D background;

  out vec4 fragColor;

  vec2 pixelate(vec2 uv, vec2 tex_size){
    return floor(uv * tex_size) / tex_size;
  }

  uint unpack_tile(vec2 uv){
    ivec2 grid_pos = ivec2(grid_size * uv);
    uint grid_idx = uint(grid_pos.y * grid_size + grid_pos.x);
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
    if (_debug > 0){
        vec2 uv_bg;
        uv_bg = vec2(uv * grid_size * 2.0);
        uv_bg.x = floor(uv_bg.x / grid_size);
        uv_bg.y = floor(uv_bg.y / grid_size);
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

#[derive(Debug)]
struct PaintBrush {
    tile: Tile,
    _drawing: bool,
    size: i16,
    max_size: i16,
    sampler: RasterPaintBrushPattern,
    pattern: RasterPaintBrushPattern,
}

impl Default for PaintBrush {
    fn default() -> Self {
        PaintBrush {
            tile: Tile::Sand,
            _drawing: false,
            size: 1,
            max_size: 64,
            sampler: RasterPaintBrushPattern::empty(),
            pattern: RasterPaintBrushPattern::circle(),
        }
    }
}

impl PaintBrush {
    pub fn new(tile: Tile, size: i16, max_size: i16, pattern: RasterPaintBrushPattern) -> Self {
        let mut sampler = RasterPaintBrushPattern::empty();

        PaintBrush::resize_sampler(size, &mut sampler, &pattern);

        Self {
            tile,
            _drawing: false,
            size,
            max_size,
            sampler: sampler,
            pattern,
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
            PaintBrush::resize_sampler(size, &mut self.sampler, &self.pattern);
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

    pub fn resize_sampler(
        size: i16,
        sampler: &mut RasterPaintBrushPattern,
        pattern: &RasterPaintBrushPattern,
    ) {
        sampler.size = size;
        sampler.data.resize((size * size) as usize, 0);

        for y in 0..size {
            let sample_y: usize = ((y as f32 / size as f32) * pattern.size as f32).floor() as usize;
            for x in 0..size {
                let sample_x: usize =
                    ((x as f32 / size as f32) * pattern.size as f32).floor() as usize;
                sampler.data[(y * size + x) as usize] =
                    pattern.data[sample_y * pattern.size as usize + sample_x];
            }
        }
    }
}

#[derive(Debug)]
struct RasterPaintBrushPattern {
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

        Self { size: size, data }
    }

    pub fn square() -> Self {
        Self {
            size: 1,
            data: vec![1; 1],
        }
    }
}

#[derive(Debug)]
struct CameraController {
    speed: f32,
    linear_damping: f32,
    zoom: f32,
    zoom_dir: f32,
    zoom_velocity: f32,
    zoom_speed: f32,
    zoom_damping: f32,
    _camera: Camera2D,
    position: Vec2,
    velocity: Vec2,
    direction: Vec2,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            speed: 1000.0,
            linear_damping: 5.0,
            zoom: 1.0,
            zoom_dir: 0.0,
            zoom_velocity: 0.0,
            zoom_speed: 100.0,
            zoom_damping: 10.0,
            _camera: Camera2D::default(),
            position: vec2(0.0, 0.0),
            direction: vec2(0.0, 0.0),
            velocity: vec2(0.0, 0.0),
        }
    }
}

impl CameraController {
    fn get(&self) -> &Camera2D {
        &self._camera
    }

    fn update(&mut self, delta: f32) {
        self.velocity += (self.speed * delta * delta) * self.direction;
        self.velocity *= 1.0 - (self.linear_damping * delta);
        let next_position = self.position + self.velocity;

        let cam_center = self.get_world_rect().center();
        if WORLD_BOUNDS.contains(cam_center + self.velocity) {
            self.position = next_position;
        } else if WORLD_BOUNDS.contains(cam_center + vec2(self.velocity.x, 0.0)) {
            self.position.x += self.velocity.x;
            self.velocity.y = 0.0;
        } else if WORLD_BOUNDS.contains(cam_center + vec2(0.0, self.velocity.y)) {
            self.position.y += self.velocity.y;
            self.velocity.x = 0.0;
        } else {
            self.velocity = vec2(0.0, 0.0);
        }

        self.direction = vec2(0.0, 0.0);

        self.zoom_velocity += self.zoom_speed * delta * delta * self.zoom_dir;
        self.zoom_velocity *= 1.0 - (self.zoom_damping * delta);
        let zoom_amp = if self.zoom > 1.0 { 2.0 } else { 1.0 };
        self.zoom += self.zoom_velocity * zoom_amp;
        self.zoom = self.zoom.clamp(0.1, 6.0);
        self.zoom_dir = 0.0;

        let mut camera = Camera2D::from_display_rect(self.get_display_rect());
        let display = get_display_rect();
        camera.viewport = Some((
            display.x as i32,
            display.y as i32,
            display.w as i32,
            display.h as i32,
        ));
        camera.zoom *= self.zoom;
        self._camera = camera;
    }

    fn reset_zoom(&mut self) {
        self.zoom = 1.0;
        self.zoom_velocity = 0.0;
    }

    fn screen_to_world(&self, pos: Vec2) -> Vec2 {
        self._camera.screen_to_world(pos)
    }

    fn world_to_screen(&self, pos: Vec2) -> Vec2 {
        self._camera.world_to_screen(pos)
    }

    fn get_display_rect(&self) -> Rect {
        Rect {
            x: self.position.x,
            y: self.position.y + SCREEN_HEIGHT as f32,
            w: SCREEN_WIDTH as f32,
            h: -SCREEN_HEIGHT as f32,
        }
    }

    fn get_world_rect(&self) -> Rect {
        Rect {
            x: self.position.x,
            y: self.position.y,
            w: SCREEN_WIDTH as f32,
            h: SCREEN_HEIGHT as f32,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct DirtyRect {
    x: i32,
    y: i32,
    x2: i32,
    y2: i32,
}

impl DirtyRect {
    const RANGE: i32 = 2;

    fn mark_dirty(&mut self, pos: IVec2) {
        let range = DirtyRect::RANGE;
        self.x = self.x.min(pos.x - range);
        self.x2 = self.x2.max(pos.x + range + 1);
        self.y = self.y.min(pos.y - range);
        self.y2 = self.y2.max(pos.y + range + 1);
    }

    fn is_empty(&self) -> bool {
        *self == DirtyRect::default()
    }

    fn reset(&mut self) {
        *self = DirtyRect::default();
    }
}

impl Default for DirtyRect {
    fn default() -> Self {
        Self {
            x: i32::MAX,
            y: i32::MAX,
            x2: i32::MIN,
            y2: i32::MIN,
        }
    }
}

pub struct RenderData {
    chunk_material: Material,
    chunk_texture: Texture2D,
    tiles_atlas: Texture2D,
    background: Texture2D,
    _debug: bool,
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
                    UniformDesc::new("_debug", UniformType::Int1),
                    UniformDesc::new("atlas_textures_size", UniformType::Int1),
                    UniformDesc::new("grid_size", UniformType::Int1),
                    UniformDesc::new("texture_scale", UniformType::Float1),
                ],
                textures: vec![
                    "tiles".to_owned(),
                    "atlas_texture".to_owned(),
                    "background".to_owned(),
                ],
            },
        )
        .expect("COULD NOT CREATE CHUNK MATERIAL");
        let tex = Texture2D::from_rgba8(
            Chunk::SIZE as u16 / 4,
            Chunk::SIZE as u16,
            &vec![0; (Chunk::SIZE * Chunk::SIZE) as usize],
        );

        Self {
            chunk_material: mat,
            chunk_texture: tex,
            tiles_atlas: Texture2D::empty(),
            background: Texture2D::empty(),
            _debug: false,
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
