use std::time::Instant;

use crate::{universe::Universe, HEIGHT, WIDTH};

pub struct Camera {
    x: f32,
    y: f32,
    zoom: f32,
    x_dest: f32,
    y_dest: f32,
    zoom_dest: f32,
    last_scroll_time: Instant,
    track_index: Option<usize>,
}

impl Camera {
    pub fn new() -> Self {
        let x = WIDTH as f32 / 2.0;
        let y = HEIGHT as f32 / 2.0;
        let zoom = 1.0;
        Camera {
            x,
            y,
            zoom,
            x_dest: x,
            y_dest: y,
            zoom_dest: zoom,
            last_scroll_time: Instant::now(),
            track_index: None,
        }
    }

    pub fn apply_zoom(&mut self, universe: &mut Universe) {
        if let Some(track_index) = self.track_index {
            self.x_dest = universe.get_particle_x(track_index).unwrap();
            self.y_dest = universe.get_particle_y(track_index).unwrap();
        }
        self.x = self.x * 0.9 + self.x_dest * 0.1;
        self.y = self.y * 0.9 + self.y_dest * 0.1;
        self.zoom = self.zoom * 0.8 + self.zoom_dest * 0.2;
        universe.zoom(self.x, self.y, self.zoom);
    }

    /// Get a mutable reference to the camera's zoom dest.
    pub fn zoom_dest_mut(&mut self) -> &mut f32 {
        &mut self.zoom_dest
    }

    pub fn zoom_dest(&self) -> f32 {
        self.zoom_dest
    }

    pub fn last_scroll_time(&self) -> Instant {
        self.last_scroll_time
    }

    /// Get a mutable reference to the camera's x dest.
    pub fn x_dest_mut(&mut self) -> &mut f32 {
        &mut self.x_dest
    }

    /// Get a mutable reference to the camera's y dest.
    pub fn y_dest_mut(&mut self) -> &mut f32 {
        &mut self.y_dest
    }
    /// Get a mutable reference to the camera's track index.
    pub fn track_index_mut(&mut self) -> &mut Option<usize> {
        &mut self.track_index
    }
}
