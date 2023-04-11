use std::fmt::Display;

pub const X_AXIS: Vec3 = Vec3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};
pub const Y_AXIS: Vec3 = Vec3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn add(self, vec: Vec3) -> Vec3 {
        Vec3::new(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }

    pub fn sub(self, vec: Vec3) -> Vec3 {
        Vec3::new(self.x - vec.x, self.y - vec.y, self.z - vec.z)
    }

    pub fn scale(self, n: f64) -> Vec3 {
        Vec3::new(self.x * n, self.y * n, self.z * n)
    }

    pub fn cross_product(self, vec: Vec3) -> Vec3 {
        let x = self.y * vec.z - self.z * vec.y;
        let y = self.z * vec.x - self.x * vec.z;
        let z = self.x * vec.y - self.y * vec.x;
        Vec3::new(x, y, z)
    }

    pub fn normalise(self) -> Vec3 {
        let length = self.length();
        self.scale(1.0 / length)
    }

    pub fn length(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.2}, {:.2}, {:.2}]", self.x, self.y, self.z)
    }
}

mod test {
    use super::*;

    #[test]
    fn add_vector() {
        let expected = Vec3::new(10.0, 10.0, 10.0);
        let initial = Vec3::new(5.0, 5.0, 5.0);
        let result = initial.add(initial);
        assert_eq!(expected, result);
    }

    #[test]
    fn sub_vector() {
        let expected = Vec3::new(10.0, 10.0, 10.0);
        let initial = Vec3::new(15.0, 15.0, 15.0);
        let result = initial.sub(Vec3::new(5.0, 5.0, 5.0));
        assert_eq!(expected, result);
    }

    #[test]
    pub fn cross_product() {
        let expected = Vec3::new(-3.0, 6.0, -3.0);
        let initial = Vec3::new(2.0, 3.0, 4.0);
        let result = initial.cross_product(Vec3::new(5.0, 6.0, 7.0));
        assert_eq!(expected, result);
    }
}
