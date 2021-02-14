use std::path::Path;

use crate::rgb_color::RawRgbColor;

const PPM_EXTENSION: &str = "ppm";

pub fn save_image(
    path_str: &str,
    rgb_list: &[RawRgbColor],
    width: u32,
    height: u32,
) -> image::ImageResult<()> {
    let buffer: Vec<_> = rgb_list
        .iter()
        .flat_map(|rgb_color| rgb_color.to_vec())
        .collect();

    let path = Path::new(path_str);
    let extension = path.extension().unwrap();

    if extension != PPM_EXTENSION {
        image::save_buffer(path, &buffer[..], width, height, image::ColorType::Rgb8)?;
    }

    let ppm_path = path.with_extension(PPM_EXTENSION);

    image::save_buffer(ppm_path, &buffer[..], width, height, image::ColorType::Rgb8)
}
