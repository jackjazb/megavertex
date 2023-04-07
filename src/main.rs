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
    let cube = [
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(0.5, -0.5, -0.5),
        Vec3::new(0.5, 0.5, -0.5),
        Vec3::new(0.5, 0.5, -0.5),
        Vec3::new(-0.5, 0.5, -0.5),
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(-0.5, -0.5, 0.5),
        Vec3::new(0.5, -0.5, 0.5),
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(-0.5, 0.5, 0.5),
        Vec3::new(-0.5, -0.5, 0.5),
        Vec3::new(-0.5, 0.5, 0.5),
        Vec3::new(-0.5, 0.5, -0.5),
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(-0.5, -0.5, 0.5),
        Vec3::new(-0.5, 0.5, 0.5),
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(0.5, 0.5, -0.5),
        Vec3::new(0.5, -0.5, -0.5),
        Vec3::new(0.5, -0.5, -0.5),
        Vec3::new(0.5, -0.5, 0.5),
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(0.5, -0.5, -0.5),
        Vec3::new(0.5, -0.5, 0.5),
        Vec3::new(0.5, -0.5, 0.5),
        Vec3::new(-0.5, -0.5, 0.5),
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(-0.5, 0.5, -0.5),
        Vec3::new(0.5, 0.5, -0.5),
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(-0.5, 0.5, 0.5),
        Vec3::new(-0.5, 0.5, -0.5),
    ];

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

    let mut degs = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        renderer.clear();

        degs = degs + deg_to_rad(2.0);

        let mat = Mat4::identity()
            .rotate(Vec3::new(0.5, 1.0, 0.0), degs)
            .scale(100.0)
            .translate(Vec3::new(300.0, 200.0, 0.0));

        for x in (0..cube.len()).step_by(3) {
            let a = mat.transform(cube[x]);
            let b = mat.transform(cube[x + 1]);
            let c = mat.transform(cube[x + 2]);

            renderer.draw_triangle([
                [a.x as i32, a.y as i32],
                [b.x as i32, b.y as i32],
                [c.x as i32, c.y as i32],
            ]);
        }

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
    rad * (180.0 / PI)
}

fn deg_to_rad(deg: f64) -> f64 {
    deg * (PI / 180.0)
}
