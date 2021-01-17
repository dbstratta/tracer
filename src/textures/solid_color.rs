use crate::{
    pdfs::{self, Pdf},
    textures::Texture,
};

pub struct SolidColor {
    pub wavelength: f32,
    pub deviation: f32,
    pub pdf: pdfs::Normal,
    pub pdf_max_value: f32,
}

impl SolidColor {
    pub fn new(wavelength: f32, deviation: f32) -> Self {
        let pdf = pdfs::Normal::new(wavelength, deviation);
        let pdf_max_value = pdf.value(wavelength);

        Self {
            wavelength,
            deviation,
            pdf,
            pdf_max_value,
        }
    }
}

impl Texture for SolidColor {
    fn probability(&self, wavelength: f32, _u: f32, _v: f32) -> f32 {
        self.pdf.value(wavelength) / self.pdf_max_value
    }
}
