use std::time::Instant;

use image::{Pixel, Rgba};
use minifb::{Key, Window, WindowOptions};

mod renderer;
use renderer::Renderer;

const WIDTH: usize = 600;
const HEIGHT: usize = 400;

struct Camera {
    position: [i32; 3], // x, y, z position of Camera in world space
}

impl Camera {
    pub fn draw_object(&mut self, vertices: Vec<[i32; 3]>) {}
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
        renderer.draw_triangle([[200, 250], [400, 250], [300, 150]]);

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
