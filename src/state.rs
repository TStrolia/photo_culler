use eframe::{App, Frame};
use egui::{Context, TextureHandle};
use std::{path::PathBuf, collections::HashMap};
use tempfile::TempDir;

use crate::ui;
use crate::io;
use crate::cache;

pub struct PhotoCullerApp {
    pub current_path: PathBuf,
    pub images: Vec<PathBuf>,
    pub selected_image: Option<PathBuf>,
    pub current_texture: Option<TextureHandle>,

    pub temp_dir: TempDir,

    pub thumbnail_cache: HashMap<PathBuf, TextureHandle>,
}

impl Default for PhotoCullerApp {
    fn default() -> Self {
        let start_path = PathBuf::from(".");
        let found_images = io::find_images::find_images(&start_path);

        let temp_dir = tempfile::Builder::new()
            .prefix("pct")
            .tempdir()
            .expect("Failed to create temp dir");

        Self {
            current_path: start_path,
            images: found_images,
            selected_image: None,
            current_texture: None,
            temp_dir,
            thumbnail_cache: HashMap::new(),
        }
    }
}

impl App for PhotoCullerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let previous_path = self.current_path.clone();
        let previous_selection = self.selected_image.clone();

        egui::SidePanel::left("tree").show(ctx, |ui| {
            let root = PathBuf::from(".");
            ui::draw_tree::draw_tree(ui, &root, &mut self.current_path);
        });

        egui::TopBottomPanel::bottom("filmstrip").show(ctx, |ui| {
            ui::draw_filmstrip::draw_filmstrip(
                ui,
                &self.images,
                &mut self.selected_image,
                &mut self.thumbnail_cache,
                &self.temp_dir,
                ctx
            );
        });

        egui::SidePanel::right("info").show(ctx, |ui| {
            ui::draw_info::draw_info(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui::draw_viewer::draw_viewer(ui, &self.current_texture);
        });

            if self.current_path != previous_path {
            // self.current_path = previous_path.clone();
            // self.current_path = target_path;

            self.selected_image = None;
            self.current_texture = None;
            self.images = io::find_images::find_images(&self.current_path);
            self.thumbnail_cache.clear(); // Clear old thumbnails from RAM

            // 2. BATCH GENERATE (The "Freeze" Warning)
            // We loop through all found images and create the .thumb files now.
            // In a real app, this goes to a thread. Here, it will block.
            let path_ref = self.temp_dir.path();
            for img_path in &self.images {
                let thumb_path = cache::paths::get_thumb_path(img_path, path_ref);
                cache::thumbnails::generate_thumbnail(img_path, &thumb_path);
            }
        }

        if self.selected_image != previous_selection {
            if let Some(path) = &self.selected_image {
                if let Some(color_image) = io::load_image::load_image(path) {
                    self.current_texture = Some(ctx.load_texture(
                        "active_image", 
                        color_image,
                        Default::default()
                    ));
                }
            }
            else {
                self.current_texture = None;
            }
        }
    }
}