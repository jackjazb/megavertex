use crate::{Object, Vec3};

pub struct Rigidbody {
    object: Object,
    centre: Vec3,
}

impl Rigidbody {
    pub fn new(object: Object) -> Rigidbody {
        let centre: Vec3 =
            object.vertices.iter().copied().sum::<Vec3>() / object.vertices.len() as f64;
        Rigidbody { object, centre }
    }
}
