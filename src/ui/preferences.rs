use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{EguiContext, EguiContexts, EguiPlugin};

#[derive(Resource)]
struct ContextMenuState {
    show_menu: bool,
    position: egui::Pos2,
}

impl Default for ContextMenuState {
    fn default() -> Self {
        Self {
            show_menu: false,
            position: egui::Pos2::ZERO,
        }
    }
}

pub struct PreferencesPlugin;

impl Plugin for PreferencesPlugin {
  fn build(&self, app: &mut App) {
    app
        .init_resource::<ContextMenuState>()
        .add_systems(Update, (right_click_system, ui_context_menu_system));
  }
}

fn right_click_system(
    mut context_menu_state: ResMut<ContextMenuState>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        let primary_window = q_windows.single_mut();
        if let Some(cursor_position) = primary_window.cursor_position() {
            let egui_position = egui::Pos2 {
                x: cursor_position.x,
                y: primary_window.height() - (primary_window.height() - cursor_position.y),
            };

            context_menu_state.show_menu = true;
            context_menu_state.position = egui_position;
        }
    }
}

fn ui_context_menu_system(
    mut contexts: EguiContexts,
    mut context_menu_state: ResMut<ContextMenuState>,
) {
    let ctx = contexts.ctx_mut();

    if context_menu_state.show_menu {
        egui::Window::new("Context Menu")
            .fixed_pos(context_menu_state.position)
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .show(ctx, |ui| {
                if ui.button("Option 1").clicked() {
                    // Handle Option 1 action
                    context_menu_state.show_menu = false;
                }
                if ui.button("Option 2").clicked() {
                    // Handle Option 2 action
                    context_menu_state.show_menu = false;
                }
                if ui.button("Close").clicked() {
                    context_menu_state.show_menu = false;
                }
            });

        // Close the context menu if clicked elsewhere
        if ctx.input(|i| i.pointer.any_click()) && !ctx.is_pointer_over_area() {
            context_menu_state.show_menu = false;
        }
    }
}
