use bevy::prelude::*;

use super::quickmenu::QuickMenuPlugins;

pub struct UiPlugins;

impl Plugin for UiPlugins {
  fn build(&self, app: &mut App) {
    app.add_plugins(QuickMenuPlugins);
  }
}
