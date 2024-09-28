use std::sync::Arc;

use bevy::prelude::*;

use super::quickmenu::QuickMenuPlugins;

pub struct UiPlugins;

impl Plugin for UiPlugins {
  fn build(&self, app: &mut App) {
    app.add_plugins(QuickMenuPlugins);
  }
}

pub fn rich_text(
    text: impl Into<Arc<String>>,
    uniq: crate::util::Uniq,
    font: &egui::FontId,
) -> egui::WidgetText {
    egui::WidgetText::RichText(egui::RichText::new(crate::util::trans(text.into(), uniq)).font(font.clone()))
}
