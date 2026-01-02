use egui::{Ui, TextureHandle};
use std::path::PathBuf;

pub fn draw_viewer(ui: &mut Ui, texture: &Option<TextureHandle>) {
    ui.centered_and_justified(|ui| {
        match texture {
            Some(tex) => {
                ui.image(tex);
            }
            None => {
                ui.label("No image selected");
            }
        }
    });
}