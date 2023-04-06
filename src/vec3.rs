#[derive(Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn add(&mut self, vec: Vec3) {
        self.x = self.x + vec.x;
        self.y = self.y + vec.y;
        self.z = self.z + vec.z;
    }

    pub fn sub(&mut self, vec: Vec3) {
        self.x = self.x - vec.x;
        self.y = self.y - vec.y;
        self.z = self.z - vec.z;
    }

    pub fn scale(&mut self, n: f64) {
        self.x = self.x * n;
        self.y = self.y * n;
        self.z = self.z * n;
    }

    pub fn dot_product(&self, vec: Vec3) -> f64 {
        (self.x * vec.x) + (self.y * vec.y) + (self.z + vec.z)
    }

    pub fn cross_product(&self, vec: Vec3) -> Vec3 {
        let x = self.y * vec.z - self.z - vec.y;
        let y = self.z * vec.x - self.x - vec.z;
        let z = self.x * vec.y - self.y - vec.x;
        Vec3::new(x, y, z)
    }

    pub fn normalise(&mut self) {
        let length = self.length();
        self.scale(1.0 / length);
    }

    pub fn length(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }
}

mod test {
    use super::*;
}
