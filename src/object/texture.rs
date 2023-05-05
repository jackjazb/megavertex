use std::{fs::File, io};

use crate::vec::vec2::Vec2;

///
///Holds a pixel buffer, along with the dimensions of the image it represents
///
#[derive(Clone)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}

impl Texture {
    ///
    /// Sample a texture at `(x, y)`, where `x` and `y` are values between 0 and 1
    ///
    pub fn sample(&self, coords: Vec2) -> u32 {
        let x = coords.x * self.width as f64;
        let y = coords.y * self.height as f64;
        let mut i = self.width * y as usize + x as usize;
        while i > self.pixels.len() - 1 {
            i -= self.pixels.len();
        }
        self.pixels[i]
    }

    ///
    /// Loads a PNG texture from a given path into a u32 pixel buffer
    ///
    pub fn load_from(path: &str) -> Result<Texture, io::Error> {
        let decoder = png::Decoder::new(File::open(path)?);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];

        let info = reader.next_frame(&mut buf).unwrap();

        let bytes = &buf[..info.buffer_size()];
        let mut pixels: Vec<u32> = vec![];

        for i in (0..bytes.len() - 2).step_by(3) {
            // Shift some bytes around to get an 32 bit colour value
            let rgba = (bytes[i] as u32) << 16 | (bytes[i + 1] as u32) << 8 | bytes[i + 2] as u32;
            pixels.push(rgba);
        }

        Ok(Texture {
            width: info.width as usize,
            height: info.height as usize,
            pixels,
        })
    }
}
