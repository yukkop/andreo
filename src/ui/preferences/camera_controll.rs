use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{EguiContext, EguiContexts, EguiPlugin};

use crate::{
    preference::{ApplyPreferencesEvent, CameraControllPreferences, ExemptPreferencesEvent, Preferences},
    rich_text,
    ui::quickmenu::{MARGIN, MENU_WITDTH},
};

use super::{PreferencesMenu, PreferencesSubmenu};

pub struct CameraMovementPlugin;

impl Plugin for CameraMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ui_context_menu_system.run_if(in_state(PreferencesSubmenu::CameraContoll)),
        );
    }
}

fn ui_context_menu_system(
    mut contexts: EguiContexts,
    preferences_menu: Res<PreferencesMenu>,
    preferences_submenu_state: Res<State<PreferencesSubmenu>>,
    mut next_preferences_submenu_state: ResMut<NextState<PreferencesSubmenu>>,
    mut preferences: ResMut<Preferences>,
    mut apply_event: EventWriter<ApplyPreferencesEvent>,
    mut exempt_event: EventWriter<ExemptPreferencesEvent>,
) {
    let ctx = contexts.ctx_mut();

    // Get the screen rect
    let preferences_rect = preferences_menu.rect();
    let preferences_width = preferences_rect.width();
    let preferences_min_y = preferences_rect.min.y;
    let screen_rect = ctx.input(|i| i.screen_rect);
    let submenu_width = MENU_WITDTH; // Assuming submenu width is MENU_WITDTH
    let mut submenu_position = preferences_rect.min
        + egui::vec2(
            preferences_width,
            preferences_min_y - preferences_rect.min.y,
        );

    // Check if the submenu would go off-screen to the right
    if submenu_position.x + submenu_width > screen_rect.max.x {
        // Not enough space on the right, so place it to the left
        submenu_position =
            preferences_rect.min - egui::vec2(submenu_width + MARGIN.left + MARGIN.right, 0.0);

        // Ensure the submenu does not go off-screen to the left
        if submenu_position.x < screen_rect.min.x {
            submenu_position.x = screen_rect.min.x;
        }
    }

    let camera_prefs = &mut preferences.camera_controll;

    egui::Window::new("Camera Controll Menu")
        .fixed_pos(submenu_position)
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .default_width(submenu_width)
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                ui.label(rich_text!("Rotation Sensitivity"));
                ui.add(
                    egui::Slider::new(&mut camera_prefs.rotation_sensitivity, 0.0..=0.10)
                        .text(rich_text!("Rotation Sensitivity")),
                );

                ui.label(rich_text!("Pan Sensitivity"));
                ui.add(
                    egui::Slider::new(&mut camera_prefs.pan_sensitivity, 0.0..=10.0)
                        .text(rich_text!("Pan Sensitivity")),
                );

                ui.label(rich_text!("Zoom Sensitivity"));
                ui.add(
                    egui::Slider::new(&mut camera_prefs.zoom_sensitivity, 0.0..=10.0)
                        .text(rich_text!("Zoom Sensitivity")),
                );

                ui.label(rich_text!("Min Distance"));
                ui.add(
                    egui::Slider::new(&mut camera_prefs.min_distance, 0.0..=100.0)
                        .text(rich_text!("Min Distance")),
                );

                ui.label(rich_text!("Max Distance"));
                ui.add(
                    egui::Slider::new(&mut camera_prefs.max_distance, 0.0..=1000.0)
                        .text(rich_text!("Max Distance")),
                );

                ui.checkbox(&mut camera_prefs.inertia_on, rich_text!("Inertia On"));

                ui.label(rich_text!("Inertia Decrement Speed"));
                ui.add(
                    egui::Slider::new(&mut camera_prefs.inertia_decrement_speed, 0.0..=1.0)
                        .text(rich_text!("Inertia Decrement Speed")),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
                    if ui.button(rich_text!("Apply")).clicked() {
                        apply_event.send(ApplyPreferencesEvent);
                    }
                    if ui.button(rich_text!("Default")).clicked() {
                        *camera_prefs = CameraControllPreferences::default();
                    }
                });
            });
        });

    if ctx.input(|i| i.pointer.any_down()) && !ctx.is_pointer_over_area() {
        exempt_event.send(ExemptPreferencesEvent);
        next_preferences_submenu_state.set(PreferencesSubmenu::Closed);
    }
}
