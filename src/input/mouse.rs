use bevy::{prelude::*, utils::HashMap};
use strum_macros::EnumIs;

use crate::util::bhashmap_default;

const CLICK_DURATION: f32 = 0.2;

const MOUSE_BUTTONS: [MouseButton; 3] = [
  MouseButton::Left,
  MouseButton::Right,
  MouseButton::Middle,
  //MouseButton::Other(0), 
];

#[derive(Clone, EnumIs)]
enum Interaction {
  None,
  Click,
  Hold,
}

#[derive(Resource, Default)]
struct MousePressTimers {
    timers: HashMap<MouseButton, Timer>,
}

#[derive(Resource)]
pub struct ExtendedButtonInput {
    interaction: HashMap<MouseButton, Interaction>,
}

impl Default for ExtendedButtonInput {
  fn default() -> Self {
    Self { interaction: bhashmap_default(MOUSE_BUTTONS, Interaction::None)}
  }
}

impl ExtendedButtonInput {
  pub fn clicked(&self, mouse_button: MouseButton) -> bool {
    if let Some(result) = self.interaction.get(&mouse_button) {
      result.is_click()
    } else { 
      log::warn!("asked clicked from not setuped MouseButton");
      false
    }
  }

  pub fn held(&self, mouse_button: MouseButton) -> bool {
    if let Some(result) = self.interaction.get(&mouse_button) {
      result.is_hold()
    } else { 
      log::warn!("asked held from not setuped MouseButton");
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

        button_input.interaction.insert(*button, Interaction::None);

        if mouse_input.pressed(*button) {
            button_input.interaction.insert(*button, Interaction::Hold);
            if let Some(timer) = timers.timers.get_mut(button) {
                // Tick the timer for this button
                timer.tick(time.delta());
            }
        }

        if mouse_input.just_released(*button) {
            if let Some(timer) = timers.timers.remove(button) {
                if !timer.finished() {
                    button_input.interaction.insert(*button, Interaction::Click);
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
