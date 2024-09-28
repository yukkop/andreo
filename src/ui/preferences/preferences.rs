use bevy::prelude::*;
use super::camera_controll::CameraMovementPlugin;

pub struct PreferencesPlugins;

impl Plugin for PreferencesPlugins {
  fn build(&self, app: &mut App) {
    app
        .add_plugins(CameraMovementPlugin);
  }
}
