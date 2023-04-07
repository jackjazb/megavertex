use std::{error::Error, f64::consts::PI, time::SystemTime};

use minifb::{Key, Window, WindowOptions};

mod renderer;
use renderer::Renderer;

mod vec3;
use vec3::Vec3;

mod mat4;
use mat4::Mat4;

const WIDTH: usize = 600;
const HEIGHT: usize = 400;
const SPEED: f64 = 0.3;

struct Camera {
    pos: Vec3,
}

fn main() -> Result<(), Box<dyn Error>> {
    // minifb window setup.
    let mut window = Window::new(
        "test window - esc exits",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )?;
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Renderer and camera setup
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    let mut camera = Camera {
        pos: Vec3::new(0.0, 0.0, 10.0),
    };

    let cubes = [
        Vec3::new(-0.5, 0.0, -1.0),
        Vec3::new(2.0, 0.0, -2.0),
        Vec3::new(0.5, 0.0, -5.0),
    ];

    let mut degs = 0.0;

    // Keep track of delta time for animation smoothing.
    let mut start = SystemTime::now();
    let mut end = SystemTime::now();
    let mut delta: f64;

    // Main loop.
    while window.is_open() && !window.is_key_down(Key::Escape) {
        delta = (end.duration_since(start)?.as_millis() as f64) / 30.0;
        start = SystemTime::now();
        renderer.clear();

        if window.is_key_down(Key::W) {
            camera.pos.z = camera.pos.z - SPEED * delta;
        }
        if window.is_key_down(Key::A) {
            camera.pos.x = camera.pos.x + SPEED * delta / 2.0;
        }
        if window.is_key_down(Key::S) {
            camera.pos.z = camera.pos.z + SPEED * delta;
        }
        if window.is_key_down(Key::D) {
            camera.pos.x = camera.pos.x - SPEED * delta / 2.0;
        }

        degs = degs + deg_to_rad(4.0 * delta);

        let cube_vert_trans = Mat4::identity()
            .rotate(Vec3::new(0.0, 1.0, 0.0), PI / 2.0)
            .rotate(Vec3::new(0.0, 1.0, 0.0), degs)
            .scale(0.2);

        let mut world_to_camera = camera.pos.clone();
        world_to_camera.scale(-1.0);
        println!("{:?}", camera.pos);
        // Draw lots of three vertices as triangles from the cube vertex list.
        let mut tri_buffer: Vec<Vec3> = vec![];
        for &cube_pos in cubes.iter() {
            let cube = cube();

            for x in 0..cube.len() {
                let mut point = cube_vert_trans.transform(cube[x]);
                let translation = Mat4::identity()
                    .translate(cube_pos)
                    .translate(camera.pos)
                    .translate(Vec3::new(0.0, degs.sin().abs() - 0.5, -2.0));
                point = translation.transform(point);
                point.scale(1.0 / point.z);
                tri_buffer.push(point);

                if tri_buffer.len() > 2 {
                    renderer.draw_triangle(tri_buffer);

                    // Reset triangle buffer
                    tri_buffer = vec![];
                }
            }
        }

        // to draw a pixel from an image
        // buffer[i] = rgba_to_u32(img.get_pixel(x as u32, y as u32));

        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();
        end = SystemTime::now();
    }
    Ok(())
}

// fn rgba_to_u32(pixel: &Rgba<u8>) -> u32 {
//     let channels = pixel.channels();
//     let (r, g, b) = (channels[0] as u32, channels[1] as u32, channels[2] as u32);
//     (r << 16) | (g << 8) | b
// }

fn deg_to_rad(deg: f64) -> f64 {
    deg * (PI / 180.0)
}

fn cube() -> [Vec3; 36] {
    [
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
    ]
}
