use std::fmt::{Debug, Display};

use crate::vec::vec3::Vec3;

///
/// An implementation of a 4x4 matrix. It can be used to apply transformations to vectors.
///
/// An arbitrary number of transformations can be applied fluently. For example, the following
/// applies a scaling, rotation, and translation to a vector in that order:
///
/// ```
/// Mat4::identity().translate(_).rotate(_).transform(vector)
/// ```
///
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Mat4 {
    pub m: [[f64; 4]; 4],
}

impl Mat4 {
    ///
    /// Initialises a new matrix with 1.0 as its diagonals.
    ///
    pub fn identity() -> Mat4 {
        Mat4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    ///
    /// Computes a matrix with which to translate a vector.
    ///
    pub fn translate(self, vec: Vec3) -> Mat4 {
        let trans_mat = Mat4 {
            m: [
                [1.0, 0.0, 0.0, vec.x],
                [0.0, 1.0, 0.0, vec.y],
                [0.0, 0.0, 1.0, vec.z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        trans_mat.mult(self)
    }

    ///
    /// Computes a matrix with which to rotate a vector.
    ///
    /// Rotation matrix found at https://en.wikipedia.org/wiki/Rotation_matrix#Rotation_matrix_from_axis_and_angle.
    ///
    pub fn rotate(self, mut axis: Vec3, theta: f64) -> Mat4 {
        axis = axis.normalise();
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
            m: [
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

    ///
    ///Multiply this by another 4x4 matrix.
    ///
    pub fn mult(self, mat: Mat4) -> Mat4 {
        let mut res = Mat4::identity();
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.m[i][k] * mat.m[k][j];
                }
                res.m[i][j] = sum;
            }
        }
        res
    }

    ///
    /// Apply this matrix as a transformation to a vector.
    ///
    pub fn transform(self, vec: Vec3) -> Vec3 {
        let vec4 = [vec.x, vec.y, vec.z, 1.0];
        let mut product = [0.0, 0.0, 0.0, 0.0];

        for i in 0..4 {
            for j in 0..4 {
                product[i] = product[i] + (vec4[j] * self.m[i][j]);
            }
        }

        Vec3::new(product[0], product[1], product[2])
    }
}

impl Display for Mat4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{:?}\n{:?}\n{:?}\n{:?}\n",
            self.m[0], self.m[1], self.m[2], self.m[3]
        )
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn create_identity_matrix() {
        let expected = Mat4 {
            m: [
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
    fn multiply_matrix() {
        let expected = Mat4 {
            m: [
                [20.0, 19.0, 16.0, 10.0],
                [24.0, 22.0, 18.0, 11.0],
                [31.0, 28.0, 22.0, 13.0],
                [40.0, 36.0, 28.0, 16.0],
            ],
        };

        let mat_a = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 2.0, 3.0, 4.0],
                [3.0, 3.0, 3.0, 4.0],
                [4.0, 4.0, 4.0, 4.0],
            ],
        };
        let mat_b = Mat4 {
            m: [
                [4.0, 3.0, 2.0, 1.0],
                [3.0, 3.0, 2.0, 1.0],
                [2.0, 2.0, 2.0, 1.0],
                [1.0, 1.0, 1.0, 1.0],
            ],
        };

        let result = mat_a.mult(mat_b);
        assert_eq!(expected, result);
    }

    #[test]
    fn transform_vector() {
        let expected = Vec3::new(4.0, 5.0, 6.0);
        let transformation_matrix = Mat4 {
            m: [
                [3.0, 0.0, 0.0, 1.0],
                [0.0, 3.0, 0.0, 2.0],
                [0.0, 0.0, 3.0, 3.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let result = transformation_matrix.transform(Vec3::new(1.0, 1.0, 1.0));

        assert_eq!(expected, result);
    }

    #[test]
    fn translate_vector() {
        let expected = Vec3::new(1.0, 2.0, 3.0);
        let result = Mat4::identity()
            .translate(Vec3::new(0.0, 1.0, 2.0))
            .transform(Vec3::new(1.0, 1.0, 1.0));

        assert_vec_eq(expected, result);
    }

    #[test]
    fn rotate_vector_about_x() {
        let expected = Vec3::new(1.0, -0.11950238978550387, 1.4091554842655063);
        let axis = Vec3::new(1.0, 0.0, 0.0);
        let result = Mat4::identity()
            .rotate(axis, 0.87)
            .transform(Vec3::new(1.0, 1.0, 1.0));

        assert_vec_eq(result, expected);
    }

    #[test]
    fn rotate_vector_about_xy() {
        let expected = Vec3::new(1.0, 1.0, -1.0);
        let axis = Vec3::new(1.0, 1.0, 0.0);
        let result = Mat4::identity()
            .rotate(axis, PI)
            .transform(Vec3::new(1.0, 1.0, 1.0));

        assert_vec_eq(expected, result)
    }

    ///
    /// Performs an equality assertion on the individual components of a vector, after rounding.
    /// This avoids test failures due to floating point inequality in more complex matrix calculations.
    ///
    pub fn assert_vec_eq(expected: Vec3, result: Vec3) {
        assert_eq!(result.x.round(), expected.x.round());
        assert_eq!(result.y.round(), expected.y.round());
        assert_eq!(result.z.round(), expected.z.round());
    }
}
