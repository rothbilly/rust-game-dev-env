use crate::sandbox::*;
use macroquack::prelude::*;

#[derive(Debug)]
pub struct CameraController {
    pub speed: f32,
    pub linear_damping: f32,
    pub zoom: f32,
    pub zoom_lerp: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    _camera: Camera2D,
    pub position: Vec2,
    pub velocity: Vec2,
    pub direction: Vec2,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            speed: 1000.0,
            linear_damping: 5.0,
            zoom: 1.0,
            zoom_lerp: 0.5,
            min_zoom: 0.5,
            max_zoom: 4.0,
            _camera: Camera2D::default(),
            position: vec2(0.0, 0.0),
            direction: vec2(0.0, 0.0),
            velocity: vec2(0.0, 0.0),
        }
    }
}

impl CameraController {
    pub fn get(&self) -> &Camera2D {
        &self._camera
    }

    pub fn update(&mut self, delta: f32) {
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

        self.zoom_lerp = self.zoom_lerp.clamp(0.0, 1.0);
        self.zoom = self.min_zoom.lerp(self.max_zoom, self.zoom_lerp);

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

    pub fn reset_zoom(&mut self) {
        self.zoom = 1.0;
        self.zoom_lerp = 0.5;
    }

    pub fn screen_to_world(&self, pos: Vec2) -> Vec2 {
        self._camera.screen_to_world(pos)
    }

    pub fn world_to_screen(&self, pos: Vec2) -> Vec2 {
        self._camera.world_to_screen(pos)
    }

    pub fn get_display_rect(&self) -> Rect {
        Rect {
            x: self.position.x,
            y: self.position.y + SCREEN_HEIGHT as f32,
            w: SCREEN_WIDTH as f32,
            h: -SCREEN_HEIGHT as f32,
        }
    }

    pub fn get_world_rect(&self) -> Rect {
        Rect {
            x: self.position.x,
            y: self.position.y,
            w: SCREEN_WIDTH as f32,
            h: SCREEN_HEIGHT as f32,
        }
    }
}
