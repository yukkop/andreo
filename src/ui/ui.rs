use bevy::prelude::*;

use super::preferences::PreferencesPlugin;

pub struct UiPlugins;

impl Plugin for UiPlugins {
  fn build(&self, app: &mut App) {
    app.add_plugins(PreferencesPlugin);
  }
}
