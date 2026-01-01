use eframe::{App, Frame};
use egui::Context;
use std::path::PathBuf;

use crate::ui;
use crate::io;

#[derive(Default)]
pub struct PhotoCullerApp {
    pub current_path: PathBuf,
    pub images: Vec<PathBuf>,
}

impl Default for PhotoCullerApp {
    fn default() -> Self {
        let start_path = PathBuf::from(".");
        let found_images = io::find_images::find_images(&start_path);

        self {
            current_path: start_path,
            images: found_images,
        }
    }
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