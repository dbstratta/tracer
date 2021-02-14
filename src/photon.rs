use crate::cie::CieTristimulus;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Photon {
    pub wavelength: f32,
    pub intensity: f32,
}

impl Photon {
    pub const fn new(wavelength: f32, intensity: f32) -> Self {
        Self {
            wavelength,
            intensity,
        }
    }

    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn cie_tristimulus(&self) -> CieTristimulus {
        CieTristimulus::from_wavelength(self.wavelength) * self.intensity
    }
}
