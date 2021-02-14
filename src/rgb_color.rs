use crate::cie::CieTristimulus;

pub const BLACK: RgbColor = RgbColor::new(0.0, 0.0, 0.0);

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct RgbColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl RgbColor {
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    pub fn from_cie_tristimulus(cie_tristimulus: CieTristimulus) -> Self {
        let red = 3.24096994 * cie_tristimulus.x
            - 1.53738318 * cie_tristimulus.y
            - 0.49861076 * cie_tristimulus.z;

        let green = -0.96924364 * cie_tristimulus.x
            + 1.8759675 * cie_tristimulus.y
            + 0.04155506 * cie_tristimulus.z;

        let blue = 0.05563008 * cie_tristimulus.x - 0.20397696 * cie_tristimulus.y
            + 1.05697151 * cie_tristimulus.z;

        Self::new(red, green, blue).cie_gamma_correct()
    }

    fn cie_gamma_correct(&self) -> Self {
        Self::new(
            Self::cie_gamma_correct_value(self.red),
            Self::cie_gamma_correct_value(self.green),
            Self::cie_gamma_correct_value(self.blue),
        )
    }

    fn cie_gamma_correct_value(value: f32) -> f32 {
        if value <= 0.0031308 {
            12.92 * value
        } else {
            1.055 * value.powf(1.0 / 2.4) - 0.055
        }
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

pub type RawRgbColor = [u8; 3];
