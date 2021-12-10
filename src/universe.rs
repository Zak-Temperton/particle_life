use std::{fmt::Display, vec};

use rand::{distributions::Distribution, prelude::ThreadRng, Rng};
use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};
use statrs::distribution::{Normal, Uniform};

use crate::{
    camera::Camera,
    max, min,
    particle::{Particle, ParticleTypes},
};

const RADIUS: f32 = 5.0;
const DIAMETER: f32 = 2.0 * RADIUS;
const R_SMOOTH: f32 = 2.0;

pub struct Universe {
    centre: Vector2,
    dimentions: Vector2,
    zoom: f32,
    wrap: bool,
    particles: Vec<Particle>,
    types: ParticleTypes,
    rng: ThreadRng,
    rand_settings: RandomSettings,
    friction: f32,
    flat_force: bool,
}

impl Universe {
    pub fn new(num_types: usize, num_particles: usize, width: f32, height: f32) -> Self {
        Universe {
            centre: Vector2::new(width * 0.5, height * 0.5),
            dimentions: Vector2::new(width, height),
            zoom: 1.0,
            wrap: true,
            types: ParticleTypes::with_len(num_types),
            particles: vec![Particle::default(); num_particles],
            rng: rand::thread_rng(),
            rand_settings: RandomSettings::new(),
            friction: 0.0,
            flat_force: false,
        }
    }

    pub fn set_population(&mut self, num_types: usize, num_particles: usize) {
        self.types.resize(num_types);
        self.particles.resize(num_particles, Particle::default());
    }

    pub fn re_seed(
        &mut self,
        attract_mean: f32,
        attract_std: f32,
        min_r: (f32, f32),
        max_r: (f32, f32),
        friction: f32,
        flat_force: bool,
    ) {
        self.friction = friction;
        self.flat_force = flat_force;
        self.rand_settings
            .re_seed(attract_mean, attract_std, min_r, max_r);
        self.set_random_types();
        self.set_random_particles();
    }

    pub fn set_random_types(&mut self) {
        let settings = &self.rand_settings;
        let rand_attr =
            Normal::new(settings.attract_mean as f64, settings.attract_std as f64).unwrap();
        let rand_min_r =
            Uniform::new(settings.min_r_lower as f64, settings.min_r_upper as f64).unwrap();
        let rand_max_r =
            Uniform::new(settings.max_r_lower as f64, settings.max_r_upper as f64).unwrap();
        let len = self.types.len() as f32;
        for i in 0..self.types.len() {
            *self.types.color_mut(i) =
                Color::new(((i as f32 / len) * 255.0) as u8, 255, self.rng.gen(), 255);
            for j in 0..self.types.len() {
                if i == j {
                    *self.types.attract_mut(i, j) = -(rand_attr.sample(&mut self.rng).abs() as f32);
                    *self.types.min_r_mut(i, j) = DIAMETER;
                } else {
                    *self.types.attract_mut(i, j) = rand_attr.sample(&mut self.rng) as f32;
                    *self.types.min_r_mut(i, j) =
                        DIAMETER.max(rand_min_r.sample(&mut self.rng) as f32);
                }
                *self.types.max_r_mut(i, j) =
                    (rand_max_r.sample(&mut self.rng) as f32).max(self.types.min_r(i, j));
                *self.types.max_r_mut(j, i) = self.types.max_r(i, j);
                *self.types.min_r_mut(j, i) = self.types.min_r(i, j);
            }
        }
    }

    pub fn set_random_particles(&mut self) {
        let rand_norm = Normal::new(0.0, 1.0).unwrap();
        for p in self.particles.iter_mut() {
            p.p_type = self.rng.gen_range(0..self.types.len()) as u8;
            p.x = (self.rng.gen_range(0.0..=1.0) * 0.5 + 0.25) * self.dimentions.x;
            p.y = (self.rng.gen_range(0.0..=1.0) * 0.5 + 0.25) * self.dimentions.y;

            p.vx = rand_norm.sample(&mut self.rng) as f32 * 0.2;
            p.vy = rand_norm.sample(&mut self.rng) as f32 * 0.2;
        }
    }

    pub fn toggle_wrap(&mut self) {
        self.wrap = !self.wrap;
    }

    pub fn step(&mut self) {
        let len = self.particles.len();
        for i in 0..len {
            let p = *self.particles.get(i).unwrap();
            for j in 0..len {
                let q = *self.particles.get(j).unwrap();

                let (mut dx, mut dy) = (q.x - p.x, q.y - p.y);
                if self.wrap {
                    if dx > self.dimentions.x * 0.5 {
                        dx -= self.dimentions.x;
                    } else if dx < -self.dimentions.x {
                        dx += self.dimentions.x;
                    }
                    if dy > self.dimentions.y * 0.5 {
                        dy -= self.dimentions.y;
                    } else if dy < -self.dimentions.y {
                        dy += self.dimentions.y;
                    }
                }
                let r2 = dx * dx + dy * dy;

                let min_r = self.types.min_r(p.p_type as usize, q.p_type as usize);
                let max_r = self.types.max_r(p.p_type as usize, q.p_type as usize);
                if r2 > max_r * max_r || r2 < 0.01 {
                    continue;
                }
                let r = r2.sqrt();
                dx /= r;
                dy /= r;
                let f = if r > min_r {
                    if self.flat_force {
                        self.types.attract(p.p_type as usize, q.p_type as usize)
                    } else {
                        let numer = 2.0 * (r - 0.5 * (max_r + min_r)).abs();
                        let denom = max_r - min_r;
                        self.types.attract(p.p_type as usize, q.p_type as usize)
                            * (1.0 - (numer / denom))
                    }
                } else {
                    R_SMOOTH * min_r * (1.0 / (min_r + R_SMOOTH) - 1.0 / (r + R_SMOOTH))
                };

                let p_mut = self.particles.get_mut(i).unwrap();
                p_mut.vx += f * dx;
                p_mut.vy += f * dy;
            }
        }

        for p in self.particles.iter_mut() {
            p.x += p.vx;
            p.y += p.vy;
            p.vx *= 1.0 - self.friction;
            p.vy *= 1.0 - self.friction;
            if self.wrap {
                if p.x < 0.0 {
                    p.x += self.dimentions.x;
                } else if p.x >= self.dimentions.x {
                    p.x -= self.dimentions.x;
                }
                if p.y < 0.0 {
                    p.y += self.dimentions.y;
                } else if p.y >= self.dimentions.y {
                    p.y -= self.dimentions.y;
                }
            } else {
                if p.x <= DIAMETER {
                    p.vx = -p.vx;
                    p.x = DIAMETER;
                } else if p.x >= self.dimentions.x - DIAMETER {
                    p.vx = -p.vx;
                    p.x = self.dimentions.x - DIAMETER;
                }
                if p.y <= DIAMETER {
                    p.vy = -p.vy;
                    p.y = DIAMETER;
                } else if p.y >= self.dimentions.y - DIAMETER {
                    p.vy = -p.vy;
                    p.y = self.dimentions.y - DIAMETER;
                }
            }
        }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle, alpha: f32) {
        for p in self.particles.iter() {
            handle.draw_circle(
                (((p.x - self.centre.x) * self.zoom) + self.dimentions.x / 2.0) as i32,
                (((p.y - self.centre.y) * self.zoom) + self.dimentions.y / 2.0) as i32,
                RADIUS * self.zoom,
                self.types.color(p.p_type as usize).fade(alpha),
            );
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> Option<usize> {
        let c = self.get_centre(x, y);
        for (i, p) in self.particles.iter().enumerate() {
            let dx = p.x - c.x;
            let dy = p.y - c.y;
            if dx * dx + dy * dy < RADIUS * RADIUS {
                return Some(i);
            }
        }
        None
    }

    pub fn get_particle_x(&self, index: usize) -> Option<f32> {
        self.particles.get(index).map(|p| p.x)
    }

    pub fn get_particle_y(&self, index: usize) -> Option<f32> {
        self.particles.get(index).map(|p| p.y)
    }

    pub fn to_centre(&self, x: usize, y: usize, cam: &mut Camera) {
        *cam.x_dest_mut() = self.centre.x + (x as f32 - self.dimentions.x / 2.0) / self.zoom;
        *cam.y_dest_mut() = self.centre.y + (y as f32 - self.dimentions.y / 2.0) / self.zoom;
    }

    pub fn get_centre(&self, x: usize, y: usize) -> Vector2 {
        Vector2::new(
            self.centre.x + (x as f32 - self.dimentions.x / 2.0) / self.zoom,
            self.centre.y + (y as f32 - self.dimentions.y / 2.0) / self.zoom,
        )
    }

    pub fn zoom(&mut self, cx: f32, cy: f32, zoom: f32) {
        self.centre.x = cx;
        self.centre.y = cy;
        self.zoom = 1.0_f32.max(zoom);
        self.centre.x = min(
            self.centre.x,
            self.dimentions.x as f32 * (1.0 - 0.5 / self.zoom),
        );
        self.centre.y = min(
            self.centre.y,
            self.dimentions.y as f32 * (1.0 - 0.5 / self.zoom),
        );
        self.centre.x = max(self.centre.x, self.dimentions.x as f32 * (0.5 / self.zoom));
        self.centre.y = max(self.centre.y, self.dimentions.y as f32 * (0.5 / self.zoom));
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut attract = String::new();
        let mut min_r = String::new();
        let mut max_r = String::new();
        let range = 0..self.types.len();
        for i in range.clone() {
            for j in range.clone() {
                attract.push_str(format!("{:.4}    ", self.types.attract(i, j)).as_str());
                min_r.push_str(format!("{:.4}    ", self.types.min_r(i, j)).as_str());
                max_r.push_str(format!("{:.4}    ", self.types.max_r(i, j)).as_str());
            }
        }
        write!(
            f,
            "\nAttract:\n{}\nMinR\n{}\nMaxR\n{}\n",
            attract, min_r, max_r
        )
    }
}

pub struct RandomSettings {
    attract_mean: f32,
    attract_std: f32,
    min_r_lower: f32,
    min_r_upper: f32,
    max_r_lower: f32,
    max_r_upper: f32,
}

impl RandomSettings {
    pub fn new() -> Self {
        RandomSettings {
            attract_mean: 0.0,
            attract_std: 0.0,
            min_r_lower: 0.0,
            min_r_upper: 0.0,
            max_r_lower: 0.0,
            max_r_upper: 0.0,
        }
    }
    pub fn re_seed(
        &mut self,
        attract_mean: f32,
        attract_std: f32,
        min_r: (f32, f32),
        max_r: (f32, f32),
    ) {
        *self = RandomSettings {
            attract_mean,
            attract_std,
            min_r_lower: min_r.0,
            min_r_upper: min_r.1,
            max_r_lower: max_r.0,
            max_r_upper: max_r.1,
        }
    }
}
