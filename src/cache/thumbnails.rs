use std::{fs, path::Path};
use egui::ColorImage;

use crate::processing::image_ops;

pub fn generate_thumbnail(original: &Path, thumb_path: &Path) -> bool {
    if thumb_path.exists() {
        return true;
    }

    if let Ok(img) = image::open(original) {
        let thumb = image_ops::resize_to_thumbnail(&img);

        if thumb.save(thumb_path).is_ok() {
            return true;
        }
    }
    false
}

pub fn load_thumbnail(thumb_path: &Path) -> Option<ColorImage> {
    if let Ok(img) = image::open(thumb_path) {
        let size = [img.width() as usize, img.height() as usize];
        let image_buffer = img.into_rgba8();
        let pixels = image_buffer.as_flat_samples();

        return Some(ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        ));
    }
    None
}