use std::sync::Arc;

use crate::{
    perlin::Perlin,
    vec3::{Color, Vec3},
};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        SolidColor { color }
    }

    fn new_from_rgb(red: f64, green: f64, blue: f64) -> Self {
        SolidColor {
            color: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        CheckerTexture { odd, even }
    }

    pub fn from_colors(c1: Color, c2: Color) -> Self {
        CheckerTexture {
            odd: Arc::new(SolidColor::new(c1)),
            even: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        let sines =
            (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(noise: Perlin, scale: f64) -> Self {
        Self { noise, scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Color {
        let turb = self.noise.turbolence(*p, 7);
        Color::ONE * 0.5 * (1.0 + (self.scale * p.z() + 10.0 * turb).sin())
    }
}
