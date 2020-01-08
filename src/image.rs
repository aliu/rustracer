use image::{ImageBuffer, Rgb};
use rayon::prelude::*;

use crate::vec3::Vec3;

pub struct Image {
    buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Image {
    pub fn new<F>(width: u32, height: u32, f: F) -> Image
    where
        F: Fn(u32, u32) -> Vec3 + Sync + Send,
    {
        let mut buffer = ImageBuffer::new(width, height);

        buffer
            .enumerate_pixels_mut()
            .par_bridge()
            .for_each(|(x, y, pixel)| {
                *pixel = Rgb(f(x, height - y - 1).to_rgb());
            });

        Image { buffer }
    }

    pub fn save(&self, path: String) -> Result<(), String> {
        match self.buffer.save(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error: {}", e)),
        }
    }
}
