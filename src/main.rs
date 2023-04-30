// Module imports
mod camera;
mod mat4;
mod object;
mod renderer;
mod vec2;
mod vec3;
mod world;

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::{error::Error, time::SystemTime};
use vec2::Vec2;

use camera::Camera;
use object::Object;
use renderer::Renderer;
use vec3::Vec3;
use world::World;

// Window/renderer parameters
const WIDTH: usize = 300;
const HEIGHT: usize = 200;

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
            scale: Scale::X2,
            scale_mode: ScaleMode::Stretch,
            topmost: false,
            none: false,
        },
    )?;

    // Renderer and camera setup
    let mut renderer = Renderer::new(WIDTH, HEIGHT);
    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 20.0));
    let mut world = World::new();

    if let Ok(cow) = Object::from_obj("./resources/dairy-cow") {
        world.add_object(cow.clone(), Vec3::new(0.0, 0.0, 0.0));
    }

    if let Ok(cube) = Object::from_obj("./resources/cube") {
        world.add_object(cube.clone(), Vec3::new(2.0, 0.0, 0.0));
        world.add_object(cube.clone(), Vec3::new(5.0, 2.0, 0.0));
        world.add_object(cube.clone(), Vec3::new(8.0, 4.0, 0.0));
    }

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

        renderer.write_text("megavertex", Vec2::new(10.0, 10.0));

        camera.render_world(&mut renderer, &world, world_time);

        window.update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)?;

        end = SystemTime::now();
    }
    Ok(())
}
