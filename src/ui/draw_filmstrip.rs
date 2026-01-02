use egui::{Context, TextureHandle, Ui, response};
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
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

    // const WINDOW_SIZE: usize = 25;

    // let current_idx = if let Some(sel) = selection {
    //     images.iter().position(|p| p ==sel).unwrap_or(0)
    // } else {
    //     0
    // };

    // let start_idx = current_idx.saturating_sub(WINDOW_SIZE);
    // let end_idx = (current_idx + WINDOW_SIZE).min(images.len());

    // let valid_window: HashSet<&PathBuf> = images[start_idx..end_idx].iter().collect();

    // thumb_cache.retain(|path, _| valid_window.contains(path));

    let mut visible_paths = HashSet::new();
    
    egui::ScrollArea::horizontal().show(ui, |ui| {
        ui.horizontal(|ui| {
            let view_rect = ui.clip_rect().expand(2500.0);

            for path in images {
                let name = path.file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_else(|| "???".into());

                let is_selected = selection.as_ref() == Some(path);

                let texture = thumb_cache.get(path);

                let response = if let Some(tex) = texture {
                    let btn = egui::Button::image(egui::Image::new(tex))
                        .selected(is_selected);

                    ui.add(btn)
                } else {
                    ui.selectable_label(is_selected, name.as_ref())
                };

                if response.clicked() {
                    *selection = Some(path.clone());
                }

                if view_rect.intersects(response.rect) {
                    visible_paths.insert(path.clone());

                    if texture.is_none() {
                        let thumb_path = cache::paths::get_thumb_path(path, temp_dir.path());

                        if let Some(img) = cache::thumbnails::load_thumbnail(&thumb_path) {
                            let tex = ctx.load_texture(name.as_ref(), img, Default::default());
                            thumb_cache.insert(path.clone(), tex);

                            ctx.request_repaint();
                        }
                    }
                }
            }
        });
    });
}