use std::f64::consts::PI;

use crate::{
    mat4::Mat4,
    vec3::{X_AXIS, Y_AXIS},
    Vec3,
};
/**
Calculates matrices relating to a 'camera' in the 3D scene.

The camera looks down the negative Z axis.
 */
#[derive(Copy, Clone)]
pub struct Camera {
    pos: Vec3,
    direction: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    rot: Vec3,
}

impl Camera {
    /**
    Creates a new Camera at the given position. The camera is looking down the negative Z axis by default.
     */
    pub fn new(pos: Vec3) -> Camera {
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

        self.right = Y_AXIS.cross_product(self.direction).normalise();
        self.up = self.direction.cross_product(self.right).normalise();
    }

    /**
    Generates a matrix to transform vectors into camera space
    */
    pub fn look_at(self) -> Mat4 {
        let rotation = Mat4 {
            m: [
                [self.right.x, self.right.y, self.right.z, 0.0],
                [self.up.x, self.up.y, self.up.z, 0.0],
                [self.direction.x, self.direction.y, self.direction.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        let translation = Mat4::identity().translate(self.pos);
        rotation.mult(translation)
    }

    /**
     * Translate the camera by X and Z on its current plane, based on the direction and right vectors.
     */
    pub fn translate(&mut self, x: f64, z: f64) {
        // Construct a translation by removing the Y component of the camera's direction and normalising the result
        let mut translation = self.direction;
        translation.y = 0.0;
        translation = translation.normalise();
        translation = translation.scale(x);

        self.pos = self.pos.add(translation);
        self.pos = self.pos.add(self.right.scale(z));
    }

    /**
    Rotate about each axis by X,Y and Z radians respectively.

    Note that:
    * X is 'Pitch'
    * Y is 'Yaw'
    */
    pub fn rotate(&mut self, offset: Vec3) {
        self.rot = self.rot.add(offset);

        if self.rot.x > PI / 2.0 {
            self.rot.x = PI / 2.0;
        }
        if self.rot.x < -PI / 2.0 {
            self.rot.x = -PI / 2.0;
        }

        self.recalc_vectors();
    }
}
