use raylib::color::Color;

pub struct ParticleTypes {
    color: Vec<Color>,
    attract: Vec<f32>,
    min_r: Vec<f32>,
    max_r: Vec<f32>,
}

impl ParticleTypes {
    pub fn with_len(len: usize) -> Self {
        ParticleTypes {
            color: vec![Color::BLACK; len],
            attract: vec![0.0; len * len],
            min_r: vec![0.0; len * len],
            max_r: vec![0.0; len * len],
        }
    }

    pub fn resize(&mut self, size: usize) {
        self.color.resize(size, Color::BLACK);
        self.attract.resize(size * size, 0.0);
        self.min_r.resize(size * size, 0.0);
        self.max_r.resize(size * size, 0.0);
    }

    pub fn len(&self) -> usize {
        self.color.len()
    }

    pub fn color(&self, i: usize) -> Option<&Color> {
        self.color.get(i)
    }

    pub fn color_mut(&mut self, i: usize) -> Option<&mut Color> {
        self.color.get_mut(i)
    }

    pub fn attract(&self, i: usize, j: usize) -> Option<&f32> {
        self.attract.get(i * self.len() + j)
    }

    pub fn attract_mut(&mut self, i: usize, j: usize) -> Option<&mut f32> {
        let len = self.len();
        self.attract.get_mut(i * len + j)
    }

    pub fn min_r(&self, i: usize, j: usize) -> Option<&f32> {
        self.min_r.get(i * self.len() + j)
    }

    pub fn min_r_mut(&mut self, i: usize, j: usize) -> Option<&mut f32> {
        let len = self.len();
        self.min_r.get_mut(i * len + j)
    }

    pub fn max_r(&self, i: usize, j: usize) -> Option<&f32> {
        self.max_r.get(i * self.len() + j)
    }

    pub fn max_r_mut(&mut self, i: usize, j: usize) -> Option<&mut f32> {
        let len = self.len();
        self.max_r.get_mut(i * len + j)
    }
}

#[derive(Default, Clone, Copy)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub p_type: u8,
}
