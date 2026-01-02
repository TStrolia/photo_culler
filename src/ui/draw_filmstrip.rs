use egui::{Ui, Context, TextureHandle};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::cache;

pub fn draw_filmstrip(
    ui: &mut Ui,
    images: &[PathBuf],
    selection: &mut Option<PathBuf>,
    thumb_cache: &mut HashMap<PathBuf, TextureHandle>,
    temp_dir: &tempfile::TempDir,
    ctx: &Context,
) {
    ui.heading("Filmstrip");
    
    egui::ScrollArea::horizontal().show(ui, |ui| {
        ui.horizontal(|ui| {
            for path in images {
                let name = path.file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_else(|| "???".into());
                let is_selected = selection.as_ref() == Some(path);

                let texture = if let Some(tex) = thumb_cache.get(path) {
                    Some(tex)
                } else {
                    let thumb_path = cache::paths::get_thumb_path(path, temp_dir.path());

                    if let Some(img) = cache::thumbnails::load_thumbnail(&thumb_path) {
                        let tex = ctx.load_texture(name.clone(), img, Default::default());

                        thumb_cache.insert(path.clone(), tex.clone());

                        thumb_cache.get(path)
                    } else {
                        None
                    }
                };

                let response = if let Some(tex) = texture {
                    ui.add(egui::ImageButton::new(tex))
                } else {
                    ui.selectable_label(is_selected, name)
                };

                if response.clicked() {
                    *selection = Some(path.clone())
                }
            }
        });
    });
}