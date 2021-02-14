use std::f32::consts::PI;
use std::ops;

use crate::helpers::random;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub const fn x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub const fn y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub const fn z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn from_polar(length: f32, azimuthal_angle: f32, polar_angle: f32) -> Self {
        let sin_polar_angle = f32::sin(polar_angle);

        Self::new(
            sin_polar_angle * f32::cos(azimuthal_angle),
            sin_polar_angle * f32::sin(azimuthal_angle),
            f32::cos(polar_angle),
        ) * length
    }

    pub fn len(self) -> f32 {
        f32::sqrt(self.len_squared())
    }

    pub fn len_squared(self) -> f32 {
        dot(self, self)
    }

    pub fn unit(self) -> Self {
        self / self.len()
    }

    pub fn random_in_sphere(radius: f32) -> Self {
        Self::from_polar(
            random(0.0..=radius),
            random(0.0..(2.0 * PI)),
            random(0.0..(2.0 * PI)),
        )
    }

    pub fn random() -> Self {
        Self::new(random(0.0..=1.0), random(0.0..=1.0), random(0.0..=1.0))
    }
}

pub fn dot(lhs: Vec3, rhs: Vec3) -> f32 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3::new(
        lhs.y * rhs.z - lhs.z * rhs.y,
        lhs.z * rhs.x - lhs.x * rhs.z,
        lhs.x * rhs.y - lhs.y * rhs.x,
    )
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) -> () {
        *self = *self + rhs;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) -> () {
        *self += -rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) -> () {
        *self = *self * rhs;
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        self * (1.0 / rhs)
    }
}

impl ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        rhs / self
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) -> () {
        *self = *self / rhs;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds: Vec3 indexed at {}", index),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds: Vec3 indexed at {}", index),
        }
    }
}
