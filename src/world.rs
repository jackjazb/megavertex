use crate::{mat4::Mat4, vec3::Vec3, Object};

pub struct World {
    pub objects: Vec<Object>,
}

impl World {
    pub fn new() -> World {
        World { objects: vec![] }
    }

    /**
    Adds an object to the world at a given location
    */
    pub fn add_object(&mut self, mut obj: Object, pos: Vec3) {
        obj.transform(Mat4::identity().translate(pos));
        self.objects.push(obj);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::{
        mat4::Mat4,
        object::{Object, Texture},
        vec3::ORIGIN,
    };

    #[test]
    fn add_object() {
        let mut world = World::new();
        let object = Object {
            vertices: vec![],
            tex_coords: vec![],
            normals: vec![],
            faces: vec![],
            texture: Texture {
                width: 0,
                height: 0,
                pixels: vec![],
            },
            transformation: Mat4::identity(),
        };
        world.add_object(object, ORIGIN);
        assert_eq!(world.objects.len(), 1);
    }
}
