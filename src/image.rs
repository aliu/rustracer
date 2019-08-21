use image::{ImageBuffer, Rgb};

use crate::vec3::Vec3;

pub struct Image {
    buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Image {
    pub fn new<F>(width: u32, height: u32, mut f: F) -> Image
    where
        F: FnMut(u32, u32) -> Vec3,
    {
        Image {
            buffer: ImageBuffer::from_fn(width, height, |x, y| {
                Rgb(f(x, height - y - 1).to_rgb()) // invert y-axis
            }),
        }
    }

    pub fn save(&self, path: String) -> Result<(), String> {
        match self.buffer.save(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error: {}", e)),
        }
    }
}
