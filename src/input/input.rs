use bevy::prelude::*;

use super::mouse::MousePlugin;

pub struct InputPlugins;

impl Plugin for InputPlugins {
  fn build(&self, app: &mut App) {
    app.add_plugins(MousePlugin);
  }
}
