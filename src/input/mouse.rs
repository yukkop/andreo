use bevy::{prelude::*, utils::HashMap};

use crate::util::bhashmap_default;

const CLICK_DURATION: f32 = 0.1;

const MOUSE_BUTTONS: [MouseButton; 3] = [
  MouseButton::Left,
  MouseButton::Right,
  MouseButton::Middle,
  //MouseButton::Other(0), 
];

#[derive(Resource, Default)]
struct MousePressTimers {
    timers: HashMap<MouseButton, Timer>,
}

#[derive(Resource)]
pub struct ExtendedButtonInput {
    clicked: HashMap<MouseButton, bool>,
}

impl Default for ExtendedButtonInput {
  fn default() -> Self {
    Self { clicked: bhashmap_default(MOUSE_BUTTONS, false)}
  }
}

impl ExtendedButtonInput {
  pub fn clicked(&self, mouse_button: MouseButton) -> bool {
    if let Some(result) = self.clicked.get(&mouse_button) {
      *result
    } else { 
      log::warn!("asked clicked from not setuped MouseButton");
      false
    }
  }
}

fn mouse_press_system(
    time: Res<Time>,
    mut timers: ResMut<MousePressTimers>,
    mut button_input: ResMut<ExtendedButtonInput>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    for button in &MOUSE_BUTTONS {
        if mouse_input.just_pressed(*button) {
            // Start a new timer for this button
            timers
                .timers
                .insert(*button, Timer::from_seconds(CLICK_DURATION, TimerMode::Once));
        }

        if mouse_input.pressed(*button) {
            if let Some(timer) = timers.timers.get_mut(button) {
                // Tick the timer for this button
                timer.tick(time.delta());
            }
        }

        button_input.clicked.insert(*button, false);
        if mouse_input.just_released(*button) {
            if let Some(timer) = timers.timers.remove(button) {
                if !timer.finished() {
                    button_input.clicked.insert(*button, true);
                }
            }
        }
    }
}

pub struct MousePlugin;

impl Plugin for MousePlugin {
  fn build(&self, app: &mut App) {
    app
        .insert_resource(MousePressTimers::default())
        .insert_resource(ExtendedButtonInput::default())
        .add_systems(First, mouse_press_system);
  }
}
