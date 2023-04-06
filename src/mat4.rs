use crate::Vec3;

#[derive(Debug, PartialEq)]
pub struct Mat4 {
    pub rows: [[f64; 4]; 4],
}

// A matrix struct, mostly for applying transformations to vectors
// new vec3 = Mat4::new().scale(3.0).rotate(0.5pi).translate(x, y, z).transform(vec3)
impl Mat4 {
    // Initialises a new matrix with 1.0 as its diagonals.
    pub fn identity() -> Mat4 {
        Mat4 {
            rows: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /**
     * Generates a matrix with which to scale a vector
     */
    pub fn scale(self, n: f64) -> Mat4 {
        let mut res = Mat4::identity();
        for i in 0..4 {
            for j in 0..4 {
                // Ignore the bottom row
                if i != 3 {
                    res.rows[i][j] = self.rows[i][j] * n;
                }
            }
        }
        return res;
    }

    /**
     * Vector translation
     */
    pub fn translate(self, vec: Vec3) -> Mat4 {
        let trans_mat = Mat4 {
            rows: [
                [1.0, 0.0, 0.0, vec.x],
                [0.0, 1.0, 0.0, vec.y],
                [0.0, 0.0, 1.0, vec.z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        return trans_mat.mult(self);
    }

    /**
     * Rotation matrix found at https://learnopengl.com/Getting-started/Transformations
     *
     * One day I will understand quaternions. Today is not that day.
     */
    pub fn rotation(self, axis: Vec3, theta: f64) -> Mat4 {
        let (x, y, z) = (axis.x, axis.y, axis.z);

        // Computes cos(t) + p^2(1-cos(t)) - t = theta, p = point
        fn diagonal(t: f64, p: f64) -> f64 {
            t.cos() + p.powf(2.0) * (1.0 - t.cos())
        }

        // Computes ab(1-cos(t)) + c sin(t)
        fn edge_plus(a: f64, b: f64, c: f64, t: f64) -> f64 {
            a * b * (1.0 - t.cos()) + c * t.sin()
        }

        fn edge_minus(a: f64, b: f64, c: f64, t: f64) -> f64 {
            a * b * (1.0 - t.cos()) - c * t.sin()
        }

        let rotation = Mat4 {
            rows: [
                [
                    diagonal(theta, x),
                    edge_minus(x, y, z, theta),
                    edge_plus(x, z, y, theta),
                    0.0,
                ],
                [
                    edge_plus(y, x, z, theta),
                    diagonal(theta, y),
                    edge_minus(y, z, x, theta),
                    0.0,
                ],
                [
                    edge_minus(z, x, y, theta),
                    edge_plus(z, y, x, theta),
                    diagonal(theta, z),
                    0.0,
                ],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        rotation.mult(self)
    }

    /**
     * Multiply by another 4x4 matrix.
     */
    pub fn mult(self, mat: Mat4) -> Mat4 {
        let mut res = Mat4::identity();
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum = sum + self.rows[i][k] * mat.rows[k][j];
                }
                res.rows[i][j] = sum;
            }
        }
        res
    }

    /**
     * Apply this matrix as a transformation to a vector.
     */
    pub fn transform(self, vec: Vec3) -> Vec3 {
        let vec4 = [vec.x, vec.y, vec.z, 1.0];
        let mut product = [0.0, 0.0, 0.0, 0.0];

        for i in 0..4 {
            for j in 0..4 {
                product[i] = product[i] + (vec4[j] * self.rows[i][j]);
            }
        }

        Vec3::new(product[0], product[1], product[2])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_identity_matrix() {
        let expected = Mat4 {
            rows: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        let result = Mat4::identity();
        assert_eq!(expected, result);
    }

    #[test]
    fn transform_vector() {
        let expected = Vec3::new(4.0, 5.0, 6.0);

        let matrix = Mat4 {
            rows: [
                [3.0, 0.0, 0.0, 1.0],
                [0.0, 3.0, 0.0, 2.0],
                [0.0, 0.0, 3.0, 3.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        let result = matrix.transform(Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(expected, result);
    }

    #[test]
    fn scale_vector() {
        let expected = Vec3::new(2.0, 2.0, 2.0);

        let vec = Vec3::new(1.0, 1.0, 1.0);
        let result = Mat4::identity().scale(2.0).transform(vec);
        assert_eq!(expected, result);
    }

    #[test]
    fn translate_vector() {
        let expected = Vec3::new(1.0, 2.0, 3.0);
        let vec = Vec3::new(1.0, 1.0, 1.0);
        let result = Mat4::identity()
            .translate(Vec3::new(0.0, 1.0, 2.0))
            .transform(vec);
        assert_eq!(expected, result);
    }

    #[test]
    fn rotate_vector() {
        let expected = Vec3::new(1.0, -0.11950238978550387, 1.4091554842655063);
        let vec = Vec3::new(1.0, 1.0, 1.0);
        let axis = Vec3::new(1.0, 0.0, 0.0);
        let result = Mat4::identity().rotation(axis, 0.87).transform(vec);
        assert_eq!(result, expected);
    }

    #[test]
    fn scale_then_translate_vector() {
        let expected = Vec3::new(2.0, 4.0, 6.0);

        let vec = Vec3::new(1.0, 1.0, 1.0);
        let result = Mat4::identity()
            .translate(Vec3::new(0.0, 1.0, 2.0))
            .scale(2.0)
            .transform(vec);
        assert_eq!(expected, result);
    }
}
