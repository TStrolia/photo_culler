use egui::Ui;
use std::path::PathBuf;

pub fn draw_filmstrip(ui: &mut Ui, images: &[PathBuf], selection: &mut Option<PathBuf>) {
    ui.heading("Filmstrip");
    
    egui::ScrollArea::horizontal().show(ui, |ui| {
        ui.horizontal(|ui| {
            for path in images {
                let name = path.file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_else(|| "???".into());

                let is_selected = selection.as_ref() == Some(path);

                if ui.selectable_label(is_selected, name).clicked() {
                    *selection = Some(path.clone());
                }

                // ui.button(name);
            }
        });
    });
}