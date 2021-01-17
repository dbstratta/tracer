pub trait Texture: Send + Sync {
    fn probability(&self, wavelength: f32, u: f32, v: f32) -> f32;
}
