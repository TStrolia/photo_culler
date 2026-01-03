use egui::Ui;
use eframe::egui;
use egui::{Frame, Sense, UiBuilder};

pub fn frame(
    ui: &mut Ui,
    image: egui::Image<'static>,
    text: &str,
    height: f32,
) -> egui::Response {
    const PADDING: f32 = 4.0;
    const RADIUS: f32 = 4.0;
    const IMAGE_HEIGHT: f32 = 128.0;

    let id = format!("{:?}_frame_btn",text);

    ui.scope_builder(
        UiBuilder::new()
            .id_salt(id)
            .sense(Sense::click()),
        |ui| {
            let (bg_fill, border_stroke, text_color) = {
                let response = ui.response();

                let visuals = ui.style().interact(&response);

                let border_stroke = egui::Stroke::new(2.0, visuals.bg_stroke.color);
                let bg_fill = if response.hovered() {
                    visuals.bg_fill.linear_multiply(1.2)
                } else {
                    visuals.bg_fill
                };

                (bg_fill, border_stroke, visuals.text_color())
            };
            

            Frame::default()
                .fill(bg_fill)
                .stroke(border_stroke)
                .corner_radius(RADIUS)
                .inner_margin(PADDING)
                .show(ui, |ui| {
                    ui.set_height(height - (PADDING * 2.0));
                    ui.set_min_width(94.0 - (PADDING * 2.0));
                    ui.set_max_width(250.0 - (PADDING * 2.0));

                    ui.vertical(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.add(
                                image.corner_radius(0.0).max_height(IMAGE_HEIGHT)
                            );
                        });

                        ui.vertical_centered(|ui| {
                            ui.label(
                                egui::RichText::new(text)
                                    .color(text_color)
                                    .size(11.0)
                            )
                        });
                    });
                });
        },
    )
    .response
}