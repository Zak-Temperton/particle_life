#![allow(dead_code)]
#![allow(unused_variables)]
use std::{fmt::Display, vec};

use rand::{distributions::Distribution, prelude::ThreadRng, Rng};
use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};
use statrs::distribution::{Normal, Uniform};

use crate::particle::{Particle, ParticleTypes};

const RADIUS: f32 = 5.0;
const DIAMETER: f32 = 2.0 * RADIUS;
const R_SMOOTH: f32 = 2.0;
pub struct Universe {
    centre: (f32, f32),
    dimentions: (f32, f32),
    zoom: f32,
    wrap: bool,
    particles: Vec<Particle>,
    types: ParticleTypes,
    rng: ThreadRng,
    rand_settings: RandomSettings,
}

impl Universe {
    pub fn new(num_types: usize, num_particles: usize, width: f32, height: f32) -> Self {
        Universe {
            centre: (width * 0.5, height * 0.5),
            dimentions: (width, height),
            zoom: 1.0,
            wrap: true,
            types: ParticleTypes::with_len(num_types),
            particles: vec![Particle::default(); num_particles],
            rng: rand::thread_rng(),
            rand_settings: RandomSettings::new(),
        }
    }

    pub fn set_population(&mut self, num_types: usize, num_particles: usize) {
        unimplemented!()
    }

    pub fn set_dimentions(&mut self, width: f32, height: f32) {
        self.dimentions = (width, height);
    }

    pub fn re_seed(
        &mut self,
        attract_mean: f32,
        attract_std: f32,
        minr_lower: f32,
        minr_upper: f32,
        maxr_lower: f32,
        maxr_upper: f32,
        friction: f32,
        flat_force: bool,
    ) {
        self.rand_settings.re_seed(
            attract_mean,
            attract_std,
            minr_lower,
            minr_upper,
            maxr_lower,
            maxr_upper,
            friction,
            flat_force,
        );
        self.set_random_types();
        self.set_random_particles();
    }

    pub fn set_random_types(&mut self) {
        let settings = &self.rand_settings;
        let rand_attr =
            Normal::new(settings.attract_mean as f64, settings.attract_std as f64).unwrap();
        let rand_minr =
            Uniform::new(settings.minr_lower as f64, settings.minr_upper as f64).unwrap();
        let rand_maxr =
            Uniform::new(settings.maxr_lower as f64, settings.maxr_upper as f64).unwrap();
        let len = self.types.len() as f32;
        for i in 0..self.types.len() {
            *self.types.color_mut(i) = Color::new(
                ((i as f32 / len) * 255.0) as u8,
                255,
                (((i & 1) as f32 * 0.5 + 0.5) * 255.0) as u8,
                255,
            );
            for j in 0..self.types.len() {
                if i == j {
                    *self.types.attract_mut(i, j) = -(rand_attr.sample(&mut self.rng).abs() as f32);
                    *self.types.min_r_mut(i, j) = DIAMETER;
                } else {
                    *self.types.attract_mut(i, j) = rand_attr.sample(&mut self.rng) as f32;
                    *self.types.min_r_mut(i, j) =
                        DIAMETER.max(rand_minr.sample(&mut self.rng) as f32);
                }
                *self.types.max_r_mut(i, j) =
                    (rand_maxr.sample(&mut self.rng) as f32).max(self.types.min_r(i, j));
                *self.types.max_r_mut(j, i) = self.types.max_r(i, j);
                *self.types.min_r_mut(j, i) = self.types.min_r(i, j);
            }
        }
    }

    pub fn set_random_particles(&mut self) {
        for p in self.particles.iter_mut() {
            p.p_type = self.rng.gen_range(0, self.types.len()) as u8;
            p.x = (self.rng.gen_range(0.0, 1.0) * 0.5 + 0.25) * self.dimentions.0;
            p.y = (self.rng.gen_range(0.0, 1.0) * 0.5 + 0.25) * self.dimentions.1;

            let rand_norm = Normal::new(0.0, 1.0).unwrap();
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
            let p = self.particles.get(i).unwrap().clone();
            for j in 0..len {
                let q = self.particles.get(j).unwrap().clone();

                let (mut dx, mut dy) = (q.x - p.x, q.y - p.y);
                if self.wrap {
                    if dx > self.dimentions.0 * 0.5 {
                        dx -= self.dimentions.0;
                    } else if dx < -self.dimentions.0 {
                        dx += self.dimentions.0;
                    }
                    if dy > self.dimentions.1 * 0.5 {
                        dy -= self.dimentions.1;
                    } else if dy < -self.dimentions.1 {
                        dy += self.dimentions.1;
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
                    if self.rand_settings.flat_force {
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
            p.vx *= 1.0 - self.rand_settings.friction;
            p.vy *= 1.0 - self.rand_settings.friction;
            if self.wrap {
                if p.x < 0.0 {
                    p.x += self.dimentions.0;
                } else if p.x >= self.dimentions.0 {
                    p.x -= self.dimentions.0;
                }
                if p.y < 0.0 {
                    p.y += self.dimentions.1;
                } else if p.y >= self.dimentions.1 {
                    p.y -= self.dimentions.1;
                }
            } else {
                if p.x <= DIAMETER {
                    p.vx = -p.vx;
                    p.x = DIAMETER;
                } else if p.x >= self.dimentions.0 - DIAMETER {
                    p.vx = -p.vx;
                    p.x = self.dimentions.0 - DIAMETER;
                }
                if p.y <= DIAMETER {
                    p.vy = -p.vy;
                    p.y = DIAMETER;
                } else if p.y >= self.dimentions.1 - DIAMETER {
                    p.vy = -p.vy;
                    p.y = self.dimentions.1 - DIAMETER;
                }
            }
        }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        for p in self.particles.iter() {
            handle.draw_circle(
                p.x as i32,
                p.y as i32,
                RADIUS,
                self.types.color(p.p_type as usize),
            );
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> Option<usize> {
        let (mut cx, mut cy) = (0.0, 0.0);
        self.to_centre(x, y, &mut cx, &mut cy);
        for (i, p) in self.particles.iter().enumerate() {
            let dx = p.x - cx;
            let dy = p.y - cy;
            if dx * dx + dy * dy < RADIUS * RADIUS {
                return Some(i);
            }
        }
        None
    }

    pub fn get_particle_x(&self, index: usize) -> Option<f32> {
        match self.particles.get(index) {
            Some(p) => Some(p.x),
            None => None,
        }
    }

    pub fn get_particle_y(&self, index: usize) -> Option<f32> {
        match self.particles.get(index) {
            Some(p) => Some(p.y),
            None => None,
        }
    }

    pub fn to_centre(&self, x: usize, y: usize, cx: &mut f32, cy: &mut f32) {
        *cx = self.centre.0 + (x as f32 - self.dimentions.0 / 2.0) / self.zoom;
        *cy = self.centre.1 + (y as f32 - self.dimentions.1 / 2.0) / self.zoom;
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
                println!("c {},{}", i, j);

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
    minr_lower: f32,
    minr_upper: f32,
    maxr_lower: f32,
    maxr_upper: f32,
    friction: f32,
    flat_force: bool,
}

impl RandomSettings {
    pub fn new() -> Self {
        RandomSettings {
            attract_mean: 0.0,
            attract_std: 0.0,
            minr_lower: 0.0,
            minr_upper: 0.0,
            maxr_lower: 0.0,
            maxr_upper: 0.0,
            friction: 0.0,
            flat_force: false,
        }
    }
    pub fn re_seed(
        &mut self,
        attract_mean: f32,
        attract_std: f32,
        minr_lower: f32,
        minr_upper: f32,
        maxr_lower: f32,
        maxr_upper: f32,
        friction: f32,
        flat_force: bool,
    ) {
        *self = RandomSettings {
            attract_mean,
            attract_std,
            minr_lower,
            minr_upper,
            maxr_lower,
            maxr_upper,
            friction,
            flat_force,
        }
    }
}
