use std::ops;

use crate::helpers::clamp;

pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    pub fn gamma(&self, value: f32) -> Self {
        Self::new(
            f32::powf(self.red, 1.0 / value),
            f32::powf(self.green, 1.0 / value),
            f32::powf(self.blue, 1.0 / value),
        )
    }

    pub fn to_rgb(&self) -> RgbColor {
        [
            f32::round(clamp(self.red, 0.0, 0.999) * 256.0) as u8,
            f32::round(clamp(self.green, 0.0, 0.999) * 256.0) as u8,
            f32::round(clamp(self.blue, 0.0, 0.999) * 256.0) as u8,
        ]
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

impl ops::Div<f32> for Color {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        self * (1.0 / rhs)
    }
}

pub type RgbColor = [u8; 3];
