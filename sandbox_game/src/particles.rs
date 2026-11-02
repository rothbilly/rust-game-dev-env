use macroquack::prelude::*;

use crate::{particles, tile::Tile};

#[derive(Copy, Clone, PartialEq)]
pub struct ParticleCreateInfo {
    pub particle_id: Tile,
    pub mass: f32,
    pub vel: Vec2,
    pub acc: Vec2,
    pub lifetime: f32,
}

pub struct ParticleSystem {
    pub particle_info: ParticleCreateInfo,
    pub len: usize,
    pub p_pos: Vec<Vec2>,
    pub p_vel: Vec<Vec2>,
    pub p_acc: Vec<Vec2>,
    pub p_time: Vec<f32>,
    pub active_idx: usize,
    pub apply_forces: fn() -> Vec2,
}

impl ParticleSystem {
    pub fn new(p_info: ParticleCreateInfo, len: usize, apply_forces: fn() -> Vec2) -> Self {
        Self {
            particle_info: p_info,
            len,
            p_pos: Vec::with_capacity(len),
            p_vel: Vec::with_capacity(len),
            p_acc: Vec::with_capacity(len),
            p_time: Vec::with_capacity(len),
            active_idx: 0_usize,
            apply_forces,
        }
    }

    pub fn init(&mut self) {
        let size = self.len;
        for _ in 0..size {
            self.p_pos.push(vec2(0.0, 0.0));
            self.p_vel.push(vec2(0.0, 0.0));
            self.p_acc.push(vec2(0.0, 0.0));
            self.p_time.push(0_f32);
        }
    }

    pub fn update(&mut self, delta: f32) {
        // if particles should expire
        let mut active_idx = self.active_idx;
        if self.particle_info.lifetime > 0.0 {
            for i in 0..active_idx {
                self.p_time[i] -= delta;
            }

            for i in 0..active_idx {
                if self.p_time[i] <= 0.0 {
                    self.deactivate(i, active_idx);
                    active_idx -= 1;
                }
            }
            self.active_idx = active_idx;
        }

        // pos first to apply initial values
        for i in 0..active_idx {
            self.p_pos[i] =
                self.p_pos[i] + self.p_vel[i] * delta + self.p_acc[i] * delta * delta * 0.5;
        }

        for i in 0..active_idx {
            let new_acc = (self.apply_forces)();
            self.p_vel[i] = self.p_vel[i] + (self.p_acc[i] + new_acc) * delta * 0.5;
            self.p_acc[i] = new_acc;
        }
    }

    pub fn activate(&mut self, pos: Vec2, vel: Vec2, acc: Vec2) {
        let p_info = self.particle_info;
        let idx = self.active_idx as usize;
        self.active_idx += 1;

        self.p_pos[idx] = pos;
        self.p_vel[idx] = vel;
        self.p_acc[idx] = acc;
        self.p_time[idx] = p_info.lifetime;
    }

    #[inline]
    pub fn deactivate(&mut self, idx: usize, active_idx: usize) {
        self.p_pos.swap(idx, active_idx);
        self.p_vel.swap(idx, active_idx);
        self.p_acc.swap(idx, active_idx);
        self.p_time.swap(idx, active_idx);
    }
}
