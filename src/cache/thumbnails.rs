use std::{
    io::BufWriter,
    fs::File,
    path::Path};
use egui::ColorImage;
use image::{
    codecs::jpeg::JpegEncoder,
    ExtendedColorType};

use crate::processing::image_ops;

pub fn generate_thumbnail(original: &Path, thumb_path: &Path) -> bool {
    if thumb_path.exists() {
        return true;
    }

    if let Ok(img) = image::open(original) {
        let thumb = image_ops::resize_to_thumbnail(&img);

        let rgb_img = thumb.to_rgb8();

        if let Ok(file) = File::create(thumb_path) {
            let ref_writer = BufWriter::new(file);

            let mut encoder = JpegEncoder::new_with_quality(ref_writer, 60);

            if encoder.encode(
                rgb_img.as_raw(),
                rgb_img.width(),
                rgb_img.height(),
                ExtendedColorType::Rgb8
            ).is_ok() {
                return true;
            }
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