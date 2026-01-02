use std::path::Path;
use egui::ColorImage;

pub fn load_image(path: &Path) -> Option<ColorImage> {
    let img = image::open(path).ok()?;

    let size = [img.width() as usize, img.height() as usize];

    let image_buffer = img.into_rgba8();

    let pixels = image_buffer.into_flat_samples();

    Some(ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}