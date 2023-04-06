use std::f64::consts::PI;

use minifb::{Key, Window, WindowOptions};

mod renderer;
use renderer::Renderer;

mod vec3;
use vec3::Vec3;

mod mat4;
use mat4::Mat4;

const WIDTH: usize = 600;
const HEIGHT: usize = 400;

struct Camera {
    position: Vec3, // x, y, z position of Camera in world space
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

    let vec1 = Vec3::new(1.0, 5.0, 0.0);
    let mut vec2 = Vec3::new(100.0, 7.0, 0.0);
    //vec1.normalise();
    vec2.normalise();
    println!("{:?}", rad_to_deg(vec1.dot_product(vec2).acos()));

    let test: Mat4 = Mat4::identity().rotate(Vec3::new(0.0, 1.0, 0.0), PI);
    //.transform(Vec3::new(1.0, 1.0, 1.0));

    println!("{:?}", test);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        renderer.clear();
        renderer.draw_triangle([[200, 250], [400, 250], [300, 150]]);

        // to draw a pixel from an image
        // buffer[i] = rgba_to_u32(img.get_pixel(x as u32, y as u32));

        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

// fn rgba_to_u32(pixel: &Rgba<u8>) -> u32 {
//     let channels = pixel.channels();
//     let (r, g, b) = (channels[0] as u32, channels[1] as u32, channels[2] as u32);
//     (r << 16) | (g << 8) | b
// }

fn rad_to_deg(rad: f64) -> f64 {
    return rad * (180.0 / std::f64::consts::PI);
}
