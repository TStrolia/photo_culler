use eframe::{App, Frame};
use egui::Context;

use crate::ui;

#[derive(Default)]
pub struct PhotoCullerApp {
    //
}

impl App for PhotoCullerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::SidePanel::left("tree").show(ctx, |ui| {
            ui::draw_tree::draw_tree(ui);
        });

        egui::TopBottomPanel::bottom("filmstrip").show(ctx, |ui| {
            ui::draw_filmstrip::draw_filmstrip(ui);
        });

        egui::SidePanel::right("info").show(ctx, |ui| {
            ui::draw_info::draw_info(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui::draw_viewer::draw_viewer(ui);
        });
    }
}