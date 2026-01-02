use image::imageops::FilterType;
use image::DynamicImage;


pub fn resize_to_thumbnail(img: &DynamicImage) -> DynamicImage {
    img.resize(10000, 128, FilterType::Nearest)
}