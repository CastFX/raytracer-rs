use image::{
    io::Reader as ImageReader, GenericImageView, ImageBuffer, Rgb, RgbImage,
};
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
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Color {
        let turb = self.noise.turbolence(*p, 7);
        Color::ONE * 0.5 * (1.0 + (self.scale * p.z() + 10.0 * turb).sin())
    }
}

pub struct ImageTexture {
    data: Option<RgbImage>,
    width: u32,
    height: u32,
    bytes_per_scanline: u32,
}

impl ImageTexture {
    const BYTES_PER_PIXEL: u32 = 3;

    pub fn from_filename(filename: String) -> ImageTexture {
        match image::open(filename) {
            Ok(img) => {
                let data = img.to_rgb8();
                let (width, height) = data.dimensions();
                let bytes_per_scanline = Self::BYTES_PER_PIXEL * width;

                ImageTexture {
                    data: Some(data),
                    width,
                    height,
                    bytes_per_scanline,
                }
            }
            _ => ImageTexture {
                data: None,
                width: 0,
                height: 0,
                bytes_per_scanline: 0,
            },
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        match &self.data {
            Some(img) => {
                let u = u.clamp(0.0, 1.0);
                let v = 1.0 - v.clamp(0.0, 1.0);

                let mut i = u * self.width as f64;
                let mut j = v * self.height as f64;

                i = i.min(self.width as f64 - 1.0);
                j = j.min(self.height as f64 - 1.0);

                let color_scale = 1.0 / 255.0;

                // eprintln!("{:?}, {:?}", i, j);
                let pixel = img.get_pixel(i as u32, j as u32);

                Color::new(
                    color_scale * pixel[0] as f64,
                    color_scale * pixel[1] as f64,
                    color_scale * pixel[2] as f64,
                )
            }

            _ => Color::new(0.0, 1.0, 1.0),
        }
    }
}
