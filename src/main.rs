use andreo::{core::LocationPlugin, CM};
use bevy::{prelude::*, render::camera::CameraPlugin};
use bevy_editor_pls::prelude::*;

fn mc(multiplier: f32) -> f32 {
  multiplier * CM
}

fn main() {
    App::new()
      .add_plugins(DefaultPlugins)
      .add_plugins(LocationPlugin)
      .add_plugins(EditorPlugin::default())
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
