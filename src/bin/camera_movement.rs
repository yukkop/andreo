use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy_editor_pls::prelude::*;

/// Constants for controlling the camera sensitivity and limits.
const ROTATION_SENSITIVITY: f32 = 0.005;
const PAN_SENSITIVITY: f32 = 0.01;
const ZOOM_SENSITIVITY: f32 = 0.5;
const MIN_DISTANCE: f32 = 1.0;
const MAX_DISTANCE: f32 = 100.0;

const INERTIA_ON: bool = true;
const INERTIA_DECREMENT_SPEED: f32 = 0.005;

/// Component to store the camera's control state.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct CameraController {
    distance: f32,
    yaw: f32,
    pitch: f32,
    point_of_view: Vec3,
    is_rotating: bool,
    is_panning: bool,
    rotating_inertia: InertiaRotation,
}

impl CameraController {
  fn rotate(&mut self, delta: Vec2) {
    self.yaw -= delta.x * ROTATION_SENSITIVITY;
    self.pitch -= delta.y * ROTATION_SENSITIVITY;
    self.pitch = self
        .pitch
        .clamp(-89.9_f32.to_radians(), 89.9_f32.to_radians());
  }
}

#[derive(Default, Reflect)]
struct InertiaRotation {
    speed: f32,
    direction: Vec2,
    /// neccessary to count enertia affter action finish
    start_second: f32,
    start_yaw: f32,
    start_pitch: f32,
}

impl InertiaRotation {
  fn is_present(&self) -> bool {
    self.speed != 0.
  }

  fn set_start(&mut self, second: f32, yaw: f32, pitch: f32) {
    self.start_second = second;
    self.start_yaw = yaw;
    self.start_pitch = pitch;
  }

  fn deplete(&mut self) {
    *self = Self::default();
  }

  fn decrement(&mut self) {
    self.speed -= INERTIA_DECREMENT_SPEED;

    if self.speed < 0. {
        self.speed = 0.;
    }
  }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .register_type::<CameraController>()
        .add_systems(Startup, setup)
        .add_systems(Update, update_camera_controller)
        .add_plugins(EditorPlugin::default())
        .run();
}

/// Setup the scene with a camera and some entities to view.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a camera with the CameraController component.
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        CameraController {
            distance: 5.0,
            yaw: 0.0,
            pitch: 0.0,
            point_of_view: Vec3::ZERO,
            is_rotating: false,
            is_panning: false,
            ..default()
        },
    ));

    // Optional: Add some entities to look at.
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::from_size(Vec3::splat(1.0)))),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..Default::default()
        }),
        ..Default::default()
    });
}

/// System to update the camera based on user input.
fn update_camera_controller(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut CameraController, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut controller, mut transform) in query.iter_mut() {
        // Handle zoom with mouse wheel.
        for event in mouse_wheel_events.read() {
            controller.distance -= event.y * ZOOM_SENSITIVITY;
            controller.distance = controller.distance.clamp(MIN_DISTANCE, MAX_DISTANCE);
        }

        // Update rotation and panning states based on mouse button input.
        controller.is_rotating = mouse_button_input.pressed(MouseButton::Right);
        controller.is_panning = mouse_button_input.pressed(MouseButton::Middle);

        // Calculate the total mouse movement since the last frame.
        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.read() {
            delta += event.delta;
        }

        if INERTIA_ON { 
          if mouse_button_input.just_pressed(MouseButton::Right) {
            controller.rotating_inertia.deplete();
            let seconds = time.elapsed().as_secs_f32();
            let yew = controller.yaw;
            let pitch = controller.pitch;
            controller.rotating_inertia.set_start(seconds, yew, pitch);
          }
          if mouse_button_input.just_released(MouseButton::Right) {
            let seconds = time.elapsed().as_secs_f32();

             // Calculate the difference in yaw and pitch
             let delta_yaw = controller.yaw - controller.rotating_inertia.start_yaw;
             let delta_pitch = controller.pitch - controller.rotating_inertia.start_pitch;

             // Calculate the rotated distance (Euclidean distance between two angles)
             let rotated_distance = (delta_yaw.powi(2) + delta_pitch.powi(2)).sqrt();

             controller.rotating_inertia.speed =
                 rotated_distance / controller.rotating_inertia.start_second;

             controller.rotating_inertia.direction = delta;
          }
          if controller.rotating_inertia.is_present() {
            let delta = controller.rotating_inertia.direction;
            controller.rotate(delta);
            controller.rotating_inertia.decrement();
          }
        }

        // Rotate the camera around the point of view.
        if controller.is_rotating {
            controller.rotate(delta);
        }

        // Pan the point of view.
        if controller.is_panning {
            // Calculate right and up vectors relative to the camera's orientation.
            let right = transform.rotation * Vec3::X;
            let up = transform.rotation * Vec3::Y * -1.;

            let distance = controller.distance;
            // Adjust the point of view based on mouse movement.
            controller.point_of_view -= (right * delta.x * PAN_SENSITIVITY
                + up * delta.y * PAN_SENSITIVITY)
                * distance / 10.0;
        }

        // Update the camera's transform to match the controller's state.
        let rotation = Quat::from_axis_angle(Vec3::Y, controller.yaw)
            * Quat::from_axis_angle(Vec3::X, controller.pitch);

        let offset = rotation * Vec3::new(0.0, 0.0, controller.distance);

        transform.translation = controller.point_of_view + offset;
        transform.look_at(controller.point_of_view, Vec3::Y);
    }
}
