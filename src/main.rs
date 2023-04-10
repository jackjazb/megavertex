use std::{error::Error, f64::consts::PI, time::SystemTime};

use minifb::{Key, Window, WindowOptions};

mod renderer;
use renderer::Renderer;

mod vec3;
use vec3::{Vec3, X_AXIS, Y_AXIS};

mod mat4;
use mat4::Mat4;

// Window/renderer parameters
const WIDTH: usize = 600;
const HEIGHT: usize = 400;

// Movement parameters
const SPEED: f64 = 0.3;
const LOOK_SPEED: f64 = 0.1;
const SENSITIVITY: f64 = 0.1;

/**
Calculates matrices relating to a 'camera' in the 3D scene.

The camera looks down the negative Z axis.
 */
#[derive(Copy, Clone)]
struct Camera {
    pos: Vec3,
    direction: Vec3,
    right: Vec3,
    up: Vec3,
    rot: Vec3,
}

impl Camera {
    fn new(pos: Vec3) -> Camera {
        let mut cam = Camera {
            pos,
            direction: Vec3::new(0.0, 0.0, 0.0),
            right: X_AXIS,
            up: Y_AXIS,
            rot: Vec3::new(0.0, -PI / 2.0, 0.0),
        };
        cam.recalc_vectors();
        cam
    }

    /**
    Recalculates the camera's 'right' and 'up' directions based on the current direction
    */
    fn recalc_vectors(&mut self) {
        self.direction.x = self.rot.y.cos() * self.rot.x.cos();
        self.direction.y = self.rot.x.sin();
        self.direction.z = self.rot.y.sin() * self.rot.x.cos();

        self.direction = self.direction.normalise();

        println!("dir: {}\nrot: {}\n\n", self.direction, self.rot);
        self.right = Y_AXIS.cross_product(self.direction).normalise();
        self.up = self.direction.cross_product(self.right).normalise();
    }

    /**
    Generates a matrix to transform vectors into camera space
    */
    fn look_at(self) -> Mat4 {
        let rotation = Mat4 {
            m: [
                [self.right.x, self.right.y, self.right.z, 0.0],
                [self.up.x, self.up.y, self.up.z, 0.0],
                [self.direction.x, self.direction.y, self.direction.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        let translation = Mat4::identity().translate(self.pos.scale(1.0));
        rotation.mult(translation)
    }

    /**
    Move the camera forward by x (or backward if x is negative)
    */
    fn forward(&mut self, x: f64) {
        self.pos = self.pos.add(self.direction.scale(x));
    }

    /**
    Move the camera right by x (or left if x is negative)
    */
    fn right(&mut self, x: f64) {
        self.pos = self.pos.add(self.right.scale(x));
    }

    /**
    Rotate about each axis by x,y,z radians.

    Note that:
    * x = pitch
    * y = yaw
    */
    fn rotate(&mut self, offset: Vec3) {
        self.rot = self.rot.add(offset);
        // if self.rot.x > PI {
        //     self.rot.x = PI;
        // }
        // if self.rot.x < -PI {
        //     self.rot.x = -PI;
        // }

        self.recalc_vectors();
    }
}

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
        WindowOptions::default(),
    )?;
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

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

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        delta = (end.duration_since(start)?.as_millis() as f64) / 30.0;
        start = SystemTime::now();
        renderer.clear();

        // Movement control
        if window.is_key_down(Key::W) {
            camera.forward(SPEED * delta);
        }
        if window.is_key_down(Key::A) {
            camera.right(-SPEED * delta);
        }
        if window.is_key_down(Key::S) {
            camera.forward(-SPEED * delta);
        }
        if window.is_key_down(Key::D) {
            camera.right(SPEED * delta);
        }

        window.get_mouse_pos(minifb::MouseMode::Clamp).map(|mouse| {
            let x_offset = (last_mouse.0 - mouse.0) * (LOOK_SPEED * SENSITIVITY * delta) as f32;
            let y_offset = (last_mouse.1 - mouse.1) * (LOOK_SPEED * SENSITIVITY * delta) as f32;
            last_mouse = mouse;
            //camera.rotate(Vec3::new(y_offset as f64, x_offset as f64, 0.0));
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
