use andreo::{core::LocationPlugin, preference::PreferencesPlugin, ui::UiPlugins, CM};
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_egui::EguiPlugin;

fn mc(multiplier: f32) -> f32 {
  multiplier * CM
}

fn main() {
    App::new()
      .add_plugins((DefaultPlugins, EguiPlugin))
      .add_plugins((PreferencesPlugin, UiPlugins))
      .add_plugins(LocationPlugin)
      .add_systems(Startup, (setup, init_plan))
      .run();
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {

}

fn init_plan(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn(PbrBundle {
    mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
    material: materials.add(Color::WHITE),
    ..default()
  });

  commands.spawn(PbrBundle {
    mesh: meshes.add(Cuboid::new(mc(27.), mc(270.), mc(675.))),
    material: materials.add(Color::WHITE),
    ..default()
  });
}
