use egui::Ui;
use std::path::PathBuf;

pub fn draw_filmstrip(ui: &mut Ui, images: &[PathBuf]) {
    ui.heading("Filmstrip");
    
    egui::ScrollArea::horizontal().show(ui, |ui| {
        ui.horizontal(|ui| {
            for path in images {
                let name = path.file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_else(|| "???".into());

                ui.button(name);
            }
        });
    });
}