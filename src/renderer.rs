use fontdue::Font;

use crate::Vec3;

const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xffffff;

pub struct Renderer {
    width: usize,
    height: usize,
    pub buffer: Vec<u32>,
    font: Font,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        // Read the font data and parse it into the font type
        let font = include_bytes!("../resources/liberation-mono.ttf") as &[u8];
        let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

        Renderer {
            width,
            height,
            buffer: vec![BLACK; width * height],
            font,
        }
    }

    pub fn write_text(&mut self, text: &str, pos: (usize, usize)) {
        let mut x_offset = pos.0;
        for char in text.chars() {
            // Rasterize and get the layout metrics for the letter 'g' at 17px.
            let (metrics, bitmap) = self.font.rasterize(char, 11.0);

            for y in 0..metrics.height {
                for x in 0..metrics.width {
                    let char_s = bitmap[x + y * metrics.width];
                    self.draw_pixel(((x + x_offset), (y + pos.1)), char_s as u32);
                }
            }
            x_offset = x_offset + metrics.advance_width as usize;
        }
    }
    // Draws a triangle from an array of 3 points.
    pub fn draw_triangle(&mut self, vertices: Vec<Vec3>) {
        let mut vert_index: usize = 0;

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
            let a = (vec1.x as isize, vec1.y as isize);
            let b = (vec2.x as isize, vec2.y as isize);
            self.draw_line(a, b);
            vert_index = vert_index + 1;
        }
    }

    fn draw_line(&mut self, a: (isize, isize), b: (isize, isize)) {
        let dx = (b.0 - a.0).abs();
        let dy = -(b.1 - a.1).abs();

        let sx = {
            if a.0 < b.0 {
                1
            } else {
                -1
            }
        };
        let sy = {
            if a.1 < b.1 {
                1
            } else {
                -1
            }
        };
        let mut err = dx + dy;

        let mut x = a.0;
        let mut y = a.1;

        loop {
            if x > 0 && y > 0 {
                self.draw_pixel((x as usize, y as usize), WHITE);
            }

            if x == b.0 && y == b.1 {
                break;
            }
            let err2 = err * 2;
            if err2 >= dy {
                if x == b.0 {
                    break;
                }
                err = err + dy;
                x = x + sx;
            }
            if err2 <= dx {
                if y == b.1 {
                    break;
                }
                err = err + dx;
                y = y + sy;
            }
        }
    }

    pub fn draw_pixel(&mut self, pos: (usize, usize), col: u32) {
        if pos.0 > self.width - 1 {
            return;
        }
        if pos.1 > self.height - 1 {
            return;
        }

        let i = (self.width * pos.1) + pos.0;
        if i < self.buffer.len() {
            self.buffer[i] = col;
        }
    }

    pub fn clear(&mut self) {
        self.buffer = vec![BLACK; self.width * self.height];
    }
}
