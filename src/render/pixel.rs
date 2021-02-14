use crate::{cie::CieTristimulus, photon::Photon, rgb_color::RgbColor};

pub struct Pixel {
    samples: Vec<Photon>,
}

impl Pixel {
    pub const fn new(samples: Vec<Photon>) -> Self {
        Self { samples }
    }

    pub fn rgb_color(&self, gamma: f32) -> RgbColor {
        RgbColor::from_cie_tristimulus(self.cie_tristimulus()).gamma_correct(gamma)
    }

    pub fn cie_tristimulus(&self) -> CieTristimulus {
        self.samples
            .iter()
            .map(|&sample| sample.cie_tristimulus())
            .fold(CieTristimulus::zero(), |sum, tristimulus| sum + tristimulus)
    }
}
