use megavertex::mat4::Mat4;
use megavertex::object::texture::Texture;
use megavertex::vec::vec2::Vec2;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::{error::Error, time::SystemTime};

use megavertex::camera::Camera;
use megavertex::object::{Face, Object};
use megavertex::renderer::Renderer;
use megavertex::vec::vec3::Vec3;
use megavertex::world::World;

// Window/renderer parameters
const WIDTH: usize = 600;
const HEIGHT: usize = 400;

// Movement parameters
const SPEED: f64 = 0.5;
const LOOK_SPEED: f64 = 0.1;

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
            scale: Scale::X1,
            scale_mode: ScaleMode::Stretch,
            topmost: false,
            none: false,
        },
    )?;

    // Renderer and camera setup
    let mut renderer = Renderer::new(WIDTH, HEIGHT);
    let mut camera = Camera::new(Vec3::new(0.0, -1.0, 20.0));
    let mut world = World::new();

    // Add models here:
    if let Ok(cow) = Object::from_obj("./resources/dairy-cow") {
        world.add_object(cow.clone(), Vec3::new(0.0, 0.0, 0.0));
    }

    if let Ok(cube) = Object::from_obj("./resources/cube") {
        world.add_object(cube.clone(), Vec3::new(2.0, 0.0, 0.0));
        world.add_object(cube.clone(), Vec3::new(5.0, 2.0, 0.0));
        world.add_object(cube.clone(), Vec3::new(8.0, 4.0, 0.0));
    }

    //let ground = gen_ground(1, 30);
    //world.add_object(ground, ORIGIN);

    // A timer that counts up from 0, representing the time within 'world'
    let mut world_time = 0.0;

    // Keep track of delta time for animation smoothing
    let mut start = SystemTime::now();
    let mut end = SystemTime::now();
    let mut delta: f64;

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update timing values
        delta = (end.duration_since(start)?.as_millis() as f64) / 30.0;
        start = SystemTime::now();

        world_time = world_time + 1.0 * delta;

        renderer.clear();

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

        renderer.write_text("megavertex", Vec2::new(5.0, 5.0), 24.0);

        camera.render_world(&mut renderer, &world, world_time);

        window.update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)?;

        end = SystemTime::now();
    }
    Ok(())
}

///
/// Generates a plane with a given size and vertex resolution.
fn gen_ground(res: isize, size: isize) -> Object {
    let tex_coords = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(1.0, 1.0),
    ];

    let texture =
        Texture::load_from("./resources/cube.png").expect("Failed to load ground texture");

    let mut vertices: Vec<Vec3> = vec![];
    let mut faces: Vec<Face> = vec![];

    // The current radius of the concentric square being added
    let mut radius = 1;

    while radius < size {
        for z in -radius..radius {
            for x in -radius..radius {
                let centre = Vec3::new(x as f64, 0.0, z as f64);
                let size = res as f64 / 2.0;

                let mut plane_vertices = vec![
                    centre + Vec3::new(-size, 0.0, size),
                    centre + Vec3::new(-size, 0.0, -size),
                    centre + Vec3::new(size, 0.0, -size),
                    centre + Vec3::new(size, 0.0, size),
                ];
                vertices.append(&mut plane_vertices);

                let len = vertices.len();

                let mut plane_faces = vec![
                    Face {
                        vertices: (len - 4, len - 3, len - 2),
                        tex_coords: (0, 1, 2),
                        normals: (0, 0, 0),
                    },
                    Face {
                        vertices: (len - 2, len - 1, len - 4),
                        tex_coords: (1, 2, 3),
                        normals: (0, 0, 0),
                    },
                ];

                faces.append(&mut plane_faces);
            }
        }
        radius += res;
    }

    Object {
        vertices,
        tex_coords,
        normals: vec![],
        faces,
        texture,
        transformation: Mat4::identity().translate(Vec3::new(0.0, -1.0, 0.0)),
    }
}
