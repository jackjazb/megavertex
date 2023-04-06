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
    pub fn draw_triangle(&mut self, vertices: [[i32; 2]; 3]) {
        let mut vert_index: usize = 0;
        while vert_index < 3 {
            let mut next_vert_index = vert_index + 1;
            if next_vert_index > 2 {
                next_vert_index = 0;
            }

            // Bresenham's line algorithm - info here:
            // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm#Algorithm_for_integer_arithmetic
            let v1 = vertices[vert_index];
            let v2 = vertices[next_vert_index];

            let dx = (v2[0] - v1[0]).abs();
            let dy = -(v2[1] - v1[1]).abs();

            let sx = {
                if v1[0] < v2[0] {
                    1
                } else {
                    -1
                }
            };
            let sy = {
                if v1[1] < v2[1] {
                    1
                } else {
                    -1
                }
            };
            let mut err = dx + dy;

            let mut x = v1[0];
            let mut y = v1[1];

            loop {
                self.draw_pixel([x, y], WHITE);
                if x == v2[0] && y == v2[1] {
                    break;
                }
                let err2 = err * 2;
                if err2 >= dy {
                    if x == v2[0] {
                        break;
                    }
                    err = err + dy;
                    x = x + sx;
                }
                if err2 <= dx {
                    if y == v2[1] {
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
