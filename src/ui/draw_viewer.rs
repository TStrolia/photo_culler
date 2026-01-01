use egui::Ui;

pub fn draw_viewer(ui: &mut Ui) {
    ui.centered_and_justified(|ui| {
        ui.heading("Main Viewer")
    });
}