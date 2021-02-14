use crate::cie::CieTristimulus;

pub fn tonemap(tristimuli: &[CieTristimulus]) -> Vec<CieTristimulus> {
    let max_intensity = calculate_max_intensity(tristimuli);

    tristimuli
        .iter()
        .map(|tristimulus| tristimulus.correct_exposure(max_intensity))
        .collect()
}

fn calculate_max_intensity(tristimuli: &[CieTristimulus]) -> f32 {
    let y_values_iter = tristimuli.iter().map(|tristimulus| tristimulus.y);

    let mean = y_values_iter.clone().sum::<f32>() / tristimuli.len() as f32;
    let deviation =
        y_values_iter.map(|y| (y - mean).powi(2)).sum::<f32>() / tristimuli.len() as f32;

    mean + deviation
}
