use bevy::prelude::*;

use super::{CameraControllPlugin, CameraController, EditorCamera};

pub struct LocationPlugin;

impl Plugin for LocationPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(CameraControllPlugin)
      .add_systems(Startup, setup);
  }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 90.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        EditorCamera,
        CameraController {
            distance: 90.0,
            yaw: 0.0,
            pitch: 0.0,
            point_of_view: Vec3::ZERO,
            is_rotating: false,
            is_panning: false,
            ..default()
        },
    ));
}

