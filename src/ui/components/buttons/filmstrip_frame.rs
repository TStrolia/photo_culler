use eframe::{egui};
use egui::{Ui, Frame, Sense, Stroke, StrokeKind, Color32, Shape, CornerRadius, Rect};

pub fn film_frame(
    ui: &mut Ui,
    image: egui::Image<'static>,
    text: &str,
    height: f32,
    selected: bool,
) -> egui::Response {
    const PADDING: f32 = 4.0;
    const RADIUS: u8 = 4;
    const IMAGE_HEIGHT: f32 = 128.0;

    let bg_shape_idx = ui.painter().add(Shape::Noop);

    let content_response = Frame::default()
        .fill(Color32::TRANSPARENT)
        .stroke(Stroke::NONE)
        .inner_margin(PADDING)
        .show(ui, |ui| {
            ui.set_height(height - (PADDING * 2.0));
            ui.set_min_width(94.0 - (PADDING * 2.0));
            ui.set_max_width(250.0 - (PADDING * 2.0));

            ui.vertical_centered(|ui| {
                    ui.add(image.max_height(IMAGE_HEIGHT));

                ui.vertical_centered(|ui|{
                    ui.label(
                        egui::RichText::new(text)
                            .size(12.0)
                            .color(if selected {
                                ui.visuals().strong_text_color()
                            } else {
                                ui.visuals().text_color()
                            })
                    )
                });
            });
        })
        .response;

    let rect = content_response.rect;

    let id = ui.make_persistent_id(text);
    let response = ui.interact(rect, id, Sense::click());

    let visuals = ui.style().interact_selectable(&response, selected);


    let bg_fill = if selected {
        ui.visuals().selection.bg_fill
    } else if response.hovered() {
        ui.visuals().widgets.hovered.bg_fill
    } else {
        ui.visuals().faint_bg_color
    };


    let border_stroke = if selected {
        ui.visuals().selection.stroke
    } else if response.hovered() {
        ui.visuals().widgets.hovered.fg_stroke
    } else {
        Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color)
    };


    ui.painter().set(
        bg_shape_idx,
        Shape::Rect(egui::epaint::RectShape::new(
            rect,
            CornerRadius::from(RADIUS), // Explicitly use CornerRadius
            bg_fill,
            border_stroke,
            StrokeKind::Inside, 
        )),
    );

    response
}