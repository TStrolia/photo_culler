use egui::Ui;

pub fn draw_filmstrip(ui: &mut Ui) {
    ui.heading("Filmstrip");
    ui.horizontal(|ui| {
        ui.label("Thumbnails appear here");
    });
}