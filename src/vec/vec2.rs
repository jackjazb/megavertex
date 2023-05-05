use std::ops::{Add, Sub};

use crate::vec::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }
    pub fn scale(self, n: f64) -> Vec2 {
        Vec2 {
            x: self.x * n,
            y: self.y * n,
        }
    }
    pub fn dot(self, vec: Vec2) -> f64 {
        self.x * vec.x + self.y * vec.y
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl From<Vec3> for Vec2 {
    fn from(vec: Vec3) -> Self {
        Vec2 { x: vec.x, y: vec.y }
    }
}
