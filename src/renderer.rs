use std::collections::{hash_map::Entry, HashMap};

use fontdue::Font;

use crate::Vec3;

const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xffffff;
const SKY: u32 = 0x0000ff;
const GROUND: u32 = 0x00ff00;

pub struct Renderer {
    width: usize,
    height: usize,
    pub buffer: Vec<u32>,
    font: Font,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        // Read the font data.
        let font = include_bytes!("../resources/liberation-mono.ttf") as &[u8];
        // Parse it into the font type.
        let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

        Renderer {
            width,
            height,
            buffer: vec![BLACK; width * height],
            font,
        }
    }

    pub fn write_text(&mut self, text: &str, pos: (u32, u32)) {
        let mut x_offset = pos.0;
        let space_offset = self.font.rasterize('a', 15.0).0.width;
        for char in text.chars() {
            // Rasterize and get the layout metrics for the letter 'g' at 17px.
            let (metrics, bitmap) = self.font.rasterize(char, 15.0);
            for y in 0..metrics.height {
                for x in 0..metrics.width {
                    let char_s = bitmap[x + y * metrics.width];
                    if char_s > 20 {
                        self.draw_pixel(
                            [(x + x_offset as usize) as i64, (y + pos.1 as usize) as i64],
                            char_s as u32,
                        );
                    }
                }
            }
            x_offset = x_offset + space_offset as u32;
        }
    }
    // Draws a triangle from an array of 3 points.
    pub fn draw_triangle(&mut self, vertices: Vec<Vec3>) {
        let mut vert_index: usize = 0;

        // This holds a map of screen Y to an ordererd vector of pixel X locations
        let mut row_pixels: HashMap<i64, Vec<i64>> = HashMap::new();

        while vert_index < 3 {
            let mut next_vert_index = vert_index + 1;
            if next_vert_index > 2 {
                next_vert_index = 0;
            }

            let mut vec1 = vertices[vert_index];
            let mut vec2 = vertices[next_vert_index];

            if vec1.z >= 0.0 || vec2.z >= 0.0 {
                return;
            }

            // Scale the vector up to screen size and align 0 to the centre of the screen
            vec1 = vec1.scale(self.width as f64);
            vec2 = vec2.scale(self.width as f64);

            let scr_centre = Vec3::new((self.width / 2) as f64, (self.height / 2) as f64, 0.0);
            vec1 = vec1.add(scr_centre);
            vec2 = vec2.add(scr_centre);

            // Bresenham's line algorithm - info here:
            // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm#Algorithm_for_integer_arithmetic
            let a = [vec1.x as i64, vec1.y as i64];
            let b = [vec2.x as i64, vec2.y as i64];
            self.draw_line(a, b, &mut row_pixels);
            vert_index = vert_index + 1;
        }

        for row in row_pixels {
            let y = row.0;
            for x in row.1[0]..row.1[row.1.len() - 1] {
                self.draw_pixel([x, y], WHITE);
            }
        }
    }

    fn draw_line(&mut self, a: [i64; 2], b: [i64; 2], row_pixels: &mut HashMap<i64, Vec<i64>>) {
        let dx = (b[0] - a[0]).abs();
        let dy = -(b[1] - a[1]).abs();

        let sx = {
            if a[0] < b[0] {
                1
            } else {
                -1
            }
        };
        let sy = {
            if a[1] < b[1] {
                1
            } else {
                -1
            }
        };
        let mut err = dx + dy;

        let mut x = a[0];
        let mut y = a[1];

        loop {
            self.draw_pixel([x, y], WHITE);

            /*
            This first checks if the row 'y' has an initialised vector of pixels. If the vector
            has already been initialised, the pixel is inserted in order using the binary search matcher.
            */
            // match row_pixels.entry(y) {
            //     Entry::Vacant(e) => {
            //         e.insert(vec![x]);
            //     }
            //     Entry::Occupied(mut e) => {
            //         let vec = e.get_mut();
            //         match vec.binary_search(&x) {
            //             Ok(_) => {}
            //             Err(pos) => e.get_mut().insert(pos, x),
            //         }
            //     }
            // }

            if x == b[0] && y == b[1] {
                break;
            }
            let err2 = err * 2;
            if err2 >= dy {
                if x == b[0] {
                    break;
                }
                err = err + dy;
                x = x + sx;
            }
            if err2 <= dx {
                if y == b[1] {
                    break;
                }
                err = err + dx;
                y = y + sy;
            }
        }
    }

    pub fn draw_pixel(&mut self, pos: [i64; 2], col: u32) {
        if pos[0] < 0 || pos[0] > self.width as i64 - 1 {
            return;
        }
        if pos[1] < 0 || pos[1] > self.height as i64 - 1 {
            return;
        }
        let i = (self.width * pos[1] as usize) + pos[0] as usize;
        if i < self.buffer.len() {
            self.buffer[i] = col;
        }
    }

    pub fn clear(&mut self) {
        self.buffer = vec![BLACK; self.width * self.height];
    }

    fn draw_sky(&mut self) {
        self.buffer = vec![SKY; (self.width * self.height) / 2];
        self.buffer
            .append(&mut vec![GROUND; ((self.width * self.height) / 2)]);
    }
}
