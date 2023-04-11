use std::{error::Error, f64::consts::PI, time::SystemTime};

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

mod renderer;
use renderer::Renderer;

mod vec3;
use vec3::{Vec3, Y_AXIS};

mod mat4;
use mat4::Mat4;

mod camera;
use camera::Camera;

// Window/renderer parameters
const WIDTH: usize = 300;
const HEIGHT: usize = 200;

// Movement parameters
const SPEED: f64 = 0.5;
const LOOK_SPEED: f64 = 0.1;
const SENSITIVITY: f64 = 0.1;

struct Object {
    pos: Vec3,
    vertices: Vec<Vec3>,
}

impl Object {
    fn new(pos: Vec3, vertices: Vec<Vec3>) -> Object {
        Object { pos, vertices }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // minifb window setup.
    let mut window = Window::new(
        "test window - esc exits",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: false,
            transparency: false,
            title: true,
            resize: false,
            scale: Scale::X2,
            scale_mode: ScaleMode::Stretch,
            topmost: false,
            none: false,
        },
    )?;

    // Renderer and camera setup
    let mut renderer = Renderer::new(WIDTH, HEIGHT);
    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 20.0));

    let scene_objects = [
        Object::new(Vec3::new(0.0, -2.0, -4.0), plane()),
        Object::new(Vec3::new(3.0, 0.0, -4.0), cube()),
        Object::new(Vec3::new(6.0, 2.0, -4.0), cube()),
        Object::new(Vec3::new(9.0, 4.0, -4.0), cube()),
        Object::new(Vec3::new(12.0, 6.0, -4.0), cube()),
    ];

    let mut counter = 0.0;

    let mut last_mouse: (f32, f32) = (0.0, 0.0);

    // Keep track of delta time for animation smoothing
    let mut start = SystemTime::now();
    let mut end = SystemTime::now();
    let mut delta: f64;
    let mut fps = 0.0;

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        delta = (end.duration_since(start)?.as_millis() as f64) / 30.0;
        fps = 1000.0 / (end.duration_since(start)?.as_millis() as f64);
        start = SystemTime::now();
        renderer.clear();
        renderer.write_text("text test", (10, 10));
        // Movement control
        if window.is_key_down(Key::W) {
            camera.translate(SPEED * delta, 0.0);
        }
        if window.is_key_down(Key::S) {
            camera.translate(-SPEED * delta, 0.0);
        }

        if window.is_key_down(Key::D) {
            camera.translate(0.0, SPEED * delta);
        }
        if window.is_key_down(Key::A) {
            camera.translate(0.0, -SPEED * delta);
        }

        window.get_mouse_pos(minifb::MouseMode::Clamp).map(|mouse| {
            let x_offset = (last_mouse.0 - mouse.0) * (LOOK_SPEED * SENSITIVITY * delta) as f32;
            let y_offset = (mouse.1 - last_mouse.1) * (LOOK_SPEED * SENSITIVITY * delta) as f32;
            last_mouse = mouse;
            // camera.rotate(Vec3::new(y_offset as f64, x_offset as f64, 0.0));
        });

        // Rotation control
        if window.is_key_down(Key::Up) {
            camera.rotate(Vec3::new(-LOOK_SPEED, 0.0, 0.0).scale(delta));
        }
        if window.is_key_down(Key::Down) {
            camera.rotate(Vec3::new(LOOK_SPEED, 0.0, 0.0).scale(delta));
        }
        if window.is_key_down(Key::Left) {
            camera.rotate(Vec3::new(0.0, LOOK_SPEED, 0.0).scale(delta));
        }
        if window.is_key_down(Key::Right) {
            camera.rotate(Vec3::new(0.0, -LOOK_SPEED, 0.0).scale(delta));
        }

        // Add some movement to the world
        counter = counter + deg_to_rad(4.0 * delta);
        let frame_transform = Mat4::identity().rotate(Y_AXIS, counter / 2.0).scale(2.0);
        //.translate(Vec3::new(0.0, -0.5 + counter.sin().abs() * 2.0, 0.0));

        // Draw each object
        for object in scene_objects.iter() {
            // Send sets of three from each objects vertices to the triangle renderer
            let mut tri_buffer: Vec<Vec3> = vec![];
            for &vertex in object.vertices.iter() {
                let bounced = frame_transform.transform(vertex);

                // Transform each object relative to world space
                let rel_to_world = Mat4::identity().translate(object.pos).transform(bounced);

                // Transform the point to camera space
                let rel_to_camera = camera.look_at().transform(rel_to_world);

                // Apply perspective projection
                let mut projected = rel_to_camera;
                let z = projected.z;
                projected = projected.scale(1.0 / rel_to_camera.z);
                projected.z = z;
                tri_buffer.push(projected);

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

fn cube() -> Vec<Vec3> {
    vec![
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

fn plane() -> Vec<Vec3> {
    vec![
        Vec3::new(-0.5, 0.0, -0.5),
        Vec3::new(0.5, 0.0, -0.5),
        Vec3::new(-0.5, 0.0, 0.5),
        Vec3::new(-0.5, 0.0, 0.5),
        Vec3::new(0.5, 0.0, 0.5),
        Vec3::new(0.5, 0.0, -0.5),
    ]
}
