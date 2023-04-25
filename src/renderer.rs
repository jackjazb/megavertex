use std::{
    cmp::{max, min},
    vec,
};

use fontdue::Font;

use crate::{object::Texture, vec2::Vec2, Vec3};

const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xffffff;
const BLUE: u32 = 0x0000aa;

const MAX_Z: f64 = 1000.0;

const WIREFRAME: bool = false;

pub struct Renderer {
    width: usize,
    height: usize,
    pub buffer: Vec<u32>,
    depth_buffer: Vec<f64>,
    font: Font,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        // Read the font data and parse it into the font type
        let font = include_bytes!("../resources/liberation-mono.ttf") as &[u8];
        let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

        let mut renderer = Renderer {
            width,
            height,
            buffer: vec![],
            depth_buffer: vec![],
            font,
        };
        renderer.clear();
        return renderer;
    }

    pub fn write_text(&mut self, text: &str, pos: Vec2) {
        let mut x_offset = pos.x;
        for char in text.chars() {
            // Rasterize and get the layout metrics for the letter 'g' at 17px.
            let (metrics, bitmap) = self.font.rasterize(char, 11.0);

            for y in 0..metrics.height {
                for x in 0..metrics.width {
                    let char_s = bitmap[x + y * metrics.width];
                    self.draw_pixel(
                        Vec3::new(x as f64 + x_offset, y as f64 + pos.y, 0.0),
                        char_s as u32,
                    );
                }
            }
            x_offset = x_offset + metrics.advance_width as f64;
        }
    }
    // Draws a triangle from an array of 3 points.
    pub fn draw_triangle(&mut self, vertices: Vec<Vec3>, texture: &Texture, tex_coords: Vec<Vec2>) {
        let col = WHITE;

        // Contains the rasterized points to be drawn
        let mut raster_points: Vec<Vec3> = vec![];

        for vec in vertices {
            if vec.z >= 0.0 {
                return;
            }
            // Scale the point up to raster space
            let scaled = vec.scale(self.width as f64);
            let scr_centre = Vec3::new(self.width as f64 / 2.0, self.height as f64 / 2.0, 0.0);
            let centred = scaled + scr_centre;

            raster_points.push(Vec3::new(centred.x, centred.y, vec.z));
        }

        // Compute the triangle's boundaries
        let x_min = min3(raster_points[0].x, raster_points[1].x, raster_points[2].x);
        let x_max = max3(raster_points[0].x, raster_points[1].x, raster_points[2].x);

        let y_min = min3(raster_points[0].y, raster_points[1].y, raster_points[2].y);
        let y_max = max3(raster_points[0].y, raster_points[1].y, raster_points[2].y);

        // TODO: This currently computes the average Z coord of all points in the triangle - it would be better to do this on a per pixel basis
        let plane_z = (raster_points[0].z + raster_points[1].z + raster_points[2].z) / 3.0;
        for x in x_min..x_max {
            for y in y_min..y_max {
                if point_in_triangle(
                    Vec3::new(x as f64, y as f64, 0.0),
                    raster_points[0],
                    raster_points[1],
                    raster_points[2],
                ) {
                    self.draw_pixel(Vec3::new(x as f64, y as f64, plane_z), col as u32);
                }
            }
        }

        if WIREFRAME {
            // Bresenham's line algorithm - info here:
            // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm#Algorithm_for_integer_arithmetic
            self.draw_line(raster_points[0], raster_points[1]);
            self.draw_line(raster_points[1], raster_points[2]);
            self.draw_line(raster_points[2], raster_points[0]);
        }
    }

    fn draw_line(&mut self, a: Vec3, b: Vec3) {
        let dx = (b.x - a.x).abs();
        let dy = -(b.y - a.y).abs();

        let sx = {
            if a.x < b.x {
                1.0
            } else {
                -1.0
            }
        };
        let sy = {
            if a.y < b.y {
                1.0
            } else {
                -1.0
            }
        };
        let mut err = dx + dy;

        let mut x = a.x;
        let mut y = a.y;

        loop {
            if x > 0.0 && y > 0.0 {
                // Wireframes ignore the depth buffer
                self.draw_pixel(Vec3::new(x, y, -100.0), BLUE);
            }

            if x == b.x && y == b.y {
                break;
            }
            let err2 = err * 2.0;
            if err2 >= dy {
                if x == b.x {
                    break;
                }
                err = err + dy;
                x = x + sx;
            }
            if err2 <= dx {
                if y == b.y {
                    break;
                }
                err = err + dx;
                y = y + sy;
            }
        }
    }

    pub fn draw_pixel(&mut self, pixel: Vec3, col: u32) {
        if pixel.x as usize > self.width - 1 {
            return;
        }
        if pixel.y as usize > self.height - 1 {
            return;
        }

        let i = (self.width * pixel.y as usize) + pixel.x as usize;
        if i < self.buffer.len() {
            if pixel.z > self.depth_buffer[i] {
                self.buffer[i] = col;
                self.depth_buffer[i] = pixel.z;
            }
        } else {
        }
    }

    pub fn clear(&mut self) {
        self.buffer = vec![BLACK; self.width * self.height];
        self.depth_buffer = vec![-MAX_Z; self.width * self.height];
    }
}

fn min3(a: f64, b: f64, c: f64) -> isize {
    min(a as isize, min(b as isize, c as isize))
}

fn max3(a: f64, b: f64, c: f64) -> isize {
    max(a as isize, max(b as isize, c as isize))
}

/**
Performs a check to see if a point is inside a triangle.

Maths found here: http://totologic.blogspot.com/2014/01/accurate-point-in-triangle-test.html
 */
fn point_in_triangle(pt: Vec3, a: Vec3, b: Vec3, c: Vec3) -> bool {
    fn sign(a: Vec3, b: Vec3, c: Vec3) -> f64 {
        ((a.x - c.x) * (b.y - c.y) - (b.x - c.x) * (a.y - c.y)) as f64
    }
    let d1 = sign(pt, a, b);
    let d2 = sign(pt, b, c);
    let d3 = sign(pt, c, a);
    let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);
    !(has_neg && has_pos)
}

fn to_barycentric(a: Vec3, b: Vec3, c: Vec3) {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn min3_is_accurate() {
        let expected: isize = 2;
        let result = min3(2.0, 3.0, 4.0);
        assert_eq!(expected, result);
    }
}
