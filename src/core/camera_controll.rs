use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};

use crate::check_zero_warn;
use crate::input::ExtendedButtonInput;
use crate::preference::{ApplyPreferencesEvent, Preferences};

#[derive(Component)]
pub struct EditorCamera;

/// Component to store the camera's control state.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct CameraController {
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub point_of_view: Vec3,
    pub is_rotating: bool,
    pub is_panning: bool,
    pub rotating_inertia: InertiaRotation,
}

impl CameraController {
  fn rotate(&mut self, delta: Vec2, sensitivity: f32) {
    self.yaw -= delta.x * sensitivity;
    self.pitch -= delta.y * sensitivity;
    self.pitch = self
        .pitch
        .clamp(-89.9_f32.to_radians(), 89.9_f32.to_radians());
  }
}

#[derive(Default, Reflect)]
struct InertiaRotation {
    speed: f32,
    direction: Vec2,
    /// neccessary to count inertia affter action finish
    start_second: f32,
    /// neccessary to count inertia affter action finish
    start_yaw: f32,
    /// neccessary to count inertia affter action finish
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

  fn decrement(&mut self, decrement_speed: f32) {
    self.speed -= decrement_speed;

    if self.speed < 0. {
        self.speed = 0.;
    }
  }
}

pub struct CameraControllPlugin;

impl Plugin for CameraControllPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<CameraController>()
      .add_systems(Update, update_camera_controller);
  }
}

/// System to update the camera based on user input.
fn update_camera_controller(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    preferences: Res<Preferences>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    extra_mouse_button_input: Res<ExtendedButtonInput>,
    mut query: Query<(&mut CameraController, &mut Transform)>,
    time: Res<Time>,
) {
    let preferences = &preferences.camera_controll;
    for (mut controller, mut transform) in query.iter_mut() {
        // Handle zoom with mouse wheel.
        for event in mouse_wheel_events.read() {
            controller.distance -= event.y * preferences.zoom_sensitivity;
            controller.distance = 
                controller.distance.clamp(preferences.min_distance, preferences.max_distance);
        }

        // Update rotation and panning states based on mouse button input.
        controller.is_rotating = extra_mouse_button_input.held(MouseButton::Right);
        controller.is_panning = extra_mouse_button_input.held(MouseButton::Middle);

        // Calculate the total mouse movement since the last frame.
        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.read() {
            delta += event.delta;
        }

        if preferences.inertia_on { 
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

             let seconds_passed = seconds - controller.rotating_inertia.start_second;

             if seconds_passed != 0. {
               controller.rotating_inertia.speed =
                   rotated_distance / seconds_passed;

               controller.rotating_inertia.direction = delta.normalize_or_zero();
             }
          }
          if controller.rotating_inertia.is_present() {
            let direction = controller.rotating_inertia.direction;
            let speed = controller.rotating_inertia.speed;

            controller.rotate(delta, preferences.rotation_sensitivity);
            controller.rotating_inertia.decrement(preferences.inertia_decrement_speed);
          }
        }

        // Rotate the camera around the point of view.
        if controller.is_rotating {
            controller.rotate(delta, preferences.rotation_sensitivity);
        }

        // Pan the point of view.
        if controller.is_panning {
            // Calculate right and up vectors relative to the camera's orientation.
            let right = transform.rotation * Vec3::X;
            let up = transform.rotation * Vec3::Y * -1.;

            let distance = controller.distance;
            // Adjust the point of view based on mouse movement.
            controller.point_of_view -= (right * delta.x * preferences.pan_sensitivity
                + up * delta.y * preferences.pan_sensitivity)
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
