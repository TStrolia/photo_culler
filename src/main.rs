mod state;
mod ui;
pub mod io;
mod cache;
mod processing;

use state::PhotoCullerApp;
use eframe::{self, NativeOptions};
use egui;

pub struct AppMetadata {
    pub title: &'static str,
    pub authors: &'static [&'static str],
}

struct WindowOptions {
    pub width: f32,
    pub height: f32,
}

impl AppMetadata {
    pub const fn new() -> Self {
        Self {
            title: "Photo Culler",
            authors: &["Thornton Strolia"],
        }
    }
}

impl WindowOptions {
    pub const fn new() -> Self {
        Self {
            width: 1280.0,
            height: 720.0,
        }
    }
} 

fn main() -> eframe::Result<()> {
    let app_meta = AppMetadata::new();
    let win_opt = WindowOptions::new();

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([win_opt.width, win_opt.height])
            .with_title(app_meta.title),
        ..Default::default()
    };

    eframe::run_native(
        app_meta.title, 
        options, 
        Box::new(|_cc| Ok(Box::new(PhotoCullerApp::default()))),
    )
}
