use crate::tile::Tile;

use crate::{dirty_rect::DirtyRect, sandbox::*};
use macroquack::prelude::*;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Debug)]
pub struct Chunk {
    // position inside sandbox's chunks collection
    pub map_position: IVec2,
    pub dirty_area: DirtyRect,
    pub neighbors: HashMap<IVec2, Rc<RefCell<Chunk>>>,
    pub tiles: Vec<u8>,

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
    pub const SIZE: i32 = 32;
    pub const SIZE_SQR: i32 = Chunk::SIZE * Chunk::SIZE;

    pub fn new() -> Self {
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

    pub fn init_maps(&mut self, neighbors: Vec<(IVec2, Rc<RefCell<Chunk>>)>) {
        for neighbor in neighbors {
            _ = self.neighbors.insert(neighbor.0, neighbor.1.clone());
        }
    }

    // Processing the queues takes about 10%-35% of the frame time, most of the time is spent sorting them
    // Should be better if we removed them and use a simple system similar to noita's
    // where we only update tiles that fall down from the bottom up, and tiles that go up get put
    // into a particle system
    pub fn process_queues(&mut self) {
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
    pub fn notify_neighbors(&mut self, tile_pos: IVec2) {
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
    pub fn process_tile(&mut self, pos: IVec2) -> bool {
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
                if (neighbors[Tile::Fire as usize] >= 1 || neighbors[Tile::Lava as usize] >= 1)
                    && roll(70.0)
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

    pub fn set_tile(&mut self, idx: usize, tile: Tile) {
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
    pub fn pos_to_idx(&self, pos: IVec2) -> usize {
        (pos.y * Chunk::SIZE + pos.x) as usize
    }

    #[inline]
    fn idx_to_pos(&self, idx: usize) -> IVec2 {
        ivec2(idx as i32 % Chunk::SIZE, (idx as i32) * (1 / Chunk::SIZE))
    }

    fn local_to_world_pos(&self, pos: IVec2) -> IVec2 {
        self.map_position * Chunk::SIZE + pos
    }

    pub fn get_world_rect(&self) -> Rect {
        Rect {
            x: (self.map_position.x * Chunk::SIZE) as f32,
            y: (self.map_position.y * Chunk::SIZE) as f32,
            w: Chunk::SIZE as f32,
            h: Chunk::SIZE as f32,
        }
    }

    pub fn is_empty(&self) -> bool {
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
