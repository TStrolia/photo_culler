use egui::{Ui};
use std::path::{Path, PathBuf};

use crate::io;

pub fn draw_tree(ui: &mut Ui, root: &Path, selected_path: &mut PathBuf) {
    let name = root.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Root".to_string());

    let is_selected = root == selected_path;

    let id = ui.make_persistent_id(root);

    egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false)
        .show_header(ui, |ui| {
            if ui.selectable_label(is_selected, name).clicked() {
                *selected_path = root.to_path_buf();
            }
        })
        .body(|ui| {
            let subdirs = io::list_dirs::list_dirs(root);
            for dir in subdirs {
                draw_tree(ui, &dir, selected_path);
            }
        });
}