use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

const UNIT: f32 = 0.1; // unit is cm

fn nu(multiplier: f32) -> f32 {
  multiplier * UNIT
}

fn main() {
    App::new()
      .add_plugins(DefaultPlugins)
      .add_plugins(EditorPlugin::default())
      .add_systems(Startup, (setup, init_plan))
      .run();
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-2.5, 5., 9.).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });
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
    mesh: meshes.add(Cuboid::new(nu(27.), nu(270.), nu(675.))),
    material: materials.add(Color::WHITE),
    ..default()
  });
}
