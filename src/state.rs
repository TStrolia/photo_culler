use eframe::{
    App,
    Frame};
use egui::{
    Context,
    TextureHandle};
use std::{
    path::PathBuf,
    collections::HashMap,
    thread};
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

        egui::TopBottomPanel::bottom("info_bar")
            .exact_height(24.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui::draw_infobar::draw_infobar(ui);
            });

        egui::SidePanel::left("file_tree")
            .default_width(325.0)
            .min_width(250.0)
            .resizable(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("file_tree_scroll")
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());

                        let root = PathBuf::from(".");
                        ui::draw_tree::draw_tree(ui, &root, &mut self.current_path);
                });
            });

        egui::TopBottomPanel::bottom("thumb_filmstrip")
            .exact_height(160.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui::draw_filmstrip::draw_filmstrip(
                    ui,
                    &self.images,
                    &mut self.selected_image,
                    &mut self.thumbnail_cache,
                    &self.temp_dir,
                    ctx
                );
            });

        egui::SidePanel::right("metadata")
            .default_width(450.0)
            .width_range(275.0..=700.0)
            .resizable(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("metadata_scroll")
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        ui::draw_metadata::draw_metadata(ui);
                });
            
            });
        

        egui::CentralPanel::default().show(ctx, |ui| {
            ui::draw_viewer::draw_viewer(ui, &self.current_texture);
        });

        if self.current_path != previous_path {
            self.selected_image = None;
            self.current_texture = None;
            self.images = io::find_images::find_images(&self.current_path);
            self.thumbnail_cache.clear(); // Clear old thumbnails from RAM

            let images_all = self.images.clone();
            let temp_path = self.temp_dir.path().to_path_buf();

            thread::spawn(move || {
                if images_all.is_empty() { return; }

                let available_cores = thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1);

                let num_threads = (available_cores as f32 * 0.8).floor() as usize;

                print!("Number of active spawn threads = {:?}", num_threads.to_string());

                let num_threads = num_threads.max(1).min(images_all.len());

                let chunk_size = (images_all.len() + num_threads -1) / num_threads;

                for chunk in images_all.chunks(chunk_size) {
                    let chunk_owned = chunk.to_vec();
                    let t_path_ref = temp_path.clone();

                    thread::spawn(move || {
                        for img_path in chunk_owned {
                            let thumb_path = cache::paths::get_thumb_path(&img_path, &t_path_ref);
                        
                            cache::thumbnails::generate_thumbnail(&img_path, &thumb_path);
                        }
                    });
                }
            });
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
            } else {
                self.current_texture = None;
            }
        }
    }
}