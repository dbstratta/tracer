use crate::materials::EmissiveMaterial;

const PLANCK_CONSTANT: f64 = 6.62_607_015_e-34;
const BOLTZMANN_CONSTANT: f64 = 1.38_064_852_e-23;
const SPEED_OF_LIGHT: f64 = 299_792_458.0;
const WIEN_CONSTANT: f64 = 2.897_771_955_e-3;

pub struct BlackBody {
    pub temperature: f32,
    pub normalization_factor: f32,
}

impl BlackBody {
    pub fn new(temperature: f32, intensity: f32) -> Self {
        Self {
            temperature,
            normalization_factor: intensity
                / boltzmann(
                    WIEN_CONSTANT * 1.0e9 / temperature as f64,
                    temperature as f64,
                ) as f32,
        }
    }
}

impl EmissiveMaterial for BlackBody {
    fn intensity(&self, wavelength: f32) -> f32 {
        boltzmann(wavelength as f64, self.temperature as f64) as f32 * self.normalization_factor
    }
}

fn boltzmann(wavelength: f64, temperature: f64) -> f64 {
    let frequency = SPEED_OF_LIGHT / (wavelength * 1.0e-9);

    (2.0 * PLANCK_CONSTANT * frequency.powi(3))
        / (SPEED_OF_LIGHT.powi(2)
            * (f64::exp(PLANCK_CONSTANT * frequency / (BOLTZMANN_CONSTANT * temperature)) - 1.0))
}
