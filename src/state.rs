use eframe::{App, Frame};
use egui::{Context, TextureHandle};
use std::path::PathBuf;

use crate::ui;
use crate::io;

pub struct PhotoCullerApp {
    pub current_path: PathBuf,
    pub images: Vec<PathBuf>,
    pub selected_image: Option<PathBuf>,
    pub current_texture: Option<TextureHandle>
}

impl Default for PhotoCullerApp {
    fn default() -> Self {
        let start_path = PathBuf::from(".");
        let found_images = io::find_images::find_images(&start_path);

        Self {
            current_path: start_path,
            images: found_images,
            selected_image: None,
            current_texture: None,
        }
    }
}

impl App for PhotoCullerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let mut target_path = self.current_path.clone();

        egui::SidePanel::left("tree").show(ctx, |ui| {
            let root = PathBuf::from(".");
            ui::draw_tree::draw_tree(ui, &root, &mut target_path);
        });

        if target_path != self.current_path {
            self.current_path = target_path;

            self.selected_image = None;
            self.current_texture = None;

            self.images = io::find_images::find_images(&self.current_path)
        }

        egui::TopBottomPanel::bottom("filmstrip").show(ctx, |ui| {
            ui::draw_filmstrip::draw_filmstrip(ui, &self.images, &mut self.selected_image);
        });

        egui::SidePanel::right("info").show(ctx, |ui| {
            ui::draw_info::draw_info(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui::draw_viewer::draw_viewer(ui, &self.current_texture);
        });

        let mut new_selection = self.selected_image.clone();

        if new_selection != self.selected_image {
            self.selected_image = new_selection;

            if let Some(path) = &self.selected_image {
                if let Some(color_image) = io::load_image::load_image(path) {
                    self.current_texture = Some(ctx.load_texture(
                        "active_image", 
                        color_image, 
                    Default::default()
                ))
                }
            }
        }
    }
}