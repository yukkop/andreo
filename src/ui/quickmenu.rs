use bevy::prelude::*;
use super::preferences::PreferencesPlugins;

pub struct QuickMenuPlugins;

impl Plugin for QuickMenuPlugins {
  fn build(&self, app: &mut App) {
    app
        .add_plugins(PreferencesPlugins);
  }
}
