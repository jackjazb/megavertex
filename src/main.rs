use std::time::Instant;

use image::{Pixel, Rgba};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

const BACKGROUND: u32 = 0x000000;
const FOREGROUND: u32 = 0xffffff;

struct Renderer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Renderer {
            width,
            height,
            buffer: vec![BACKGROUND; width * height],
        }
    }

    // pub fn draw_rect(&mut self, pos: Vec2, size: Vec2) {
    //     let start = pos.y * self.width + pos.x;
    //     let end = (pos.y * self.width) + (size.y * self.width) + (pos.x + size.y);

    //     for (i, el) in self.buffer.iter_mut().enumerate() {
    //         if i > start && i < end {
    //             if i % self.width > pos.x && i % self.width < pos.x + size.x {
    //                 *el = FOREGROUND;
    //             }
    //         }
    //     }
    // }

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

            println!("drawing line {:?} to {:?}", v1, v2);

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
                self.draw_pixel([x, y], FOREGROUND);
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
        println!("{:?}", pos);
        let i = (self.width * pos[1] as usize) + pos[0] as usize;
        if i < self.buffer.len() {
            self.buffer[i] = col;
        }
    }

    pub fn clear(&mut self) {
        self.buffer = vec![BACKGROUND; self.width * self.height];
    }
}

fn main() {
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "test window - esc exits",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // let img = image::open("./github.png")
    //     .expect("file not found")
    //     .into_rgba8();

    let mut last = Instant::now();
    let mut delta = 1;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        renderer.clear();
        renderer.draw_triangle([[50, 100], [200, 200], [300, 300]]);

        // to draw a pixel from an image
        // buffer[i] = rgba_to_u32(img.get_pixel(x as u32, y as u32));

        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();
        delta = last.elapsed().as_millis() as usize / 10;
        last = Instant::now();
    }
}

fn rgba_to_u32(pixel: &Rgba<u8>) -> u32 {
    let channels = pixel.channels();
    let (r, g, b) = (channels[0] as u32, channels[1] as u32, channels[2] as u32);
    (r << 16) | (g << 8) | b
}
