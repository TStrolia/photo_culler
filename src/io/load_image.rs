use std::path::Path;
use egui::ColorImage;

pub fn load_image(path: &Path) -> Option<ColorImage> {
    // PRINT: Verify the path we are trying to load
    println!("DEBUG: Attempting to load: {:?}", path);

    let image_result = image::open(path);

    match image_result {
        Ok(img) => {
            println!("DEBUG: Success! Image decoded. Size: {}x{}", img.width(), img.height());
            let size = [img.width() as usize, img.height() as usize];
            let image_buffer = img.into_rgba8();
            let pixels = image_buffer.as_flat_samples();
            
            Some(ColorImage::from_rgba_unmultiplied(
                size, 
                pixels.as_slice(),
            ))
        }
        Err(err) => {
            // PRINT: Why did it fail?
            println!("DEBUG: Failed to load image! Error: {}", err);
            None
        }
    }
}