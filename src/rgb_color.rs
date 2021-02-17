use std::ops;

use serde::{Deserialize, Serialize};

pub const BLACK: RgbColor = RgbColor::new(0.0, 0.0, 0.0);
pub const WHITE: RgbColor = RgbColor::new(1.0, 1.0, 1.0);

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct RgbColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl RgbColor {
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    pub fn gamma_correct(&self, value: f32) -> Self {
        Self::new(
            f32::powf(self.red, 1.0 / value),
            f32::powf(self.green, 1.0 / value),
            f32::powf(self.blue, 1.0 / value),
        )
    }

    pub fn to_raw(&self) -> RawRgbColor {
        [
            f32::round(self.red.clamp(0.0, 0.999) * 256.0) as u8,
            f32::round(self.green.clamp(0.0, 0.999) * 256.0) as u8,
            f32::round(self.blue.clamp(0.0, 0.999) * 256.0) as u8,
        ]
    }
}

impl ops::Add for RgbColor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl ops::Mul for RgbColor {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

impl ops::Mul<f32> for RgbColor {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

impl ops::Mul<RgbColor> for f32 {
    type Output = RgbColor;

    fn mul(self, rhs: RgbColor) -> RgbColor {
        rhs * self
    }
}

impl ops::Div<f32> for RgbColor {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        self * (1.0 / rhs)
    }
}

impl ops::Div<RgbColor> for f32 {
    type Output = RgbColor;

    fn div(self, rhs: RgbColor) -> RgbColor {
        rhs / self
    }
}

pub type RawRgbColor = [u8; 3];
