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
        obj.transformation = Mat4::identity().translate(pos);
        self.objects.push(obj);
    }
}
