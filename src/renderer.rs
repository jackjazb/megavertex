use crate::Mat4;
use crate::Vec3;

const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xffffff;

pub struct Renderer {
    width: usize,
    height: usize,
    pub buffer: Vec<u32>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Renderer {
            width,
            height,
            buffer: vec![BLACK; width * height],
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

            // Scale the vector up to screen size and align 0 to the centre of the screen
            vec1.scale(self.width as f64);
            vec2.scale(self.width as f64);

            let scr_centre = Vec3::new((self.width / 2) as f64, (self.height / 2) as f64, 0.0);
            vec1.add(scr_centre);
            vec2.add(scr_centre);

            // Bresenham's line algorithm - info here:
            // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm#Algorithm_for_integer_arithmetic
            let pix1 = [vec1.x as i32, vec1.y as i32];
            let pix2 = [vec2.x as i32, vec2.y as i32];

            let dx = (pix2[0] - pix1[0]).abs();
            let dy = -(pix2[1] - pix1[1]).abs();

            let sx = {
                if pix1[0] < pix2[0] {
                    1
                } else {
                    -1
                }
            };
            let sy = {
                if pix1[1] < pix2[1] {
                    1
                } else {
                    -1
                }
            };
            let mut err = dx + dy;

            let mut x = pix1[0];
            let mut y = pix1[1];

            loop {
                self.draw_pixel([x, y], WHITE);
                if x == pix2[0] && y == pix2[1] {
                    break;
                }
                let err2 = err * 2;
                if err2 >= dy {
                    if x == pix2[0] {
                        break;
                    }
                    err = err + dy;
                    x = x + sx;
                }
                if err2 <= dx {
                    if y == pix2[1] {
                        break;
                    }
                    err = err + dx;
                    y = y + sy;
                }
            }
            vert_index = vert_index + 1;
        }
    }

    pub fn draw_pixel(&mut self, pos: [i32; 2], col: u32) {
        if pos[0] < 0 || pos[0] > self.width as i32 - 1 {
            return;
        }
        if pos[1] < 0 || pos[1] > self.height as i32 - 1 {
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
}
