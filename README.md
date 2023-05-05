# megavertex

A 3D rendering engine with no hardware acceleration.

## Running Locally

- Install `cargo` using the `rustup` installer
- Run `cargo run --release` from the root of the project. Note that the `--release` flag is not strictly necessary, but performance will be significantly worse without it.

## Adding Models

megavertex currently supports `.obj` files. To add one: 
- Add the model file and a `.png` texture file with the same name to the `resources` directory.
- In `main.rs`, add the following in the `Add models here` section:

```
    if let Ok(object) = Object::from_obj("./resources/object-name") {
        world.add_object(object.clone(), Vec3::new(0.0, 0.0, 0.0));
    }
```

## To Do
- Physics and collisions, starting with a rigid body physics implementation
- Lighting and shading (though making this run on the CPU might be tricky)
- Skybox textures