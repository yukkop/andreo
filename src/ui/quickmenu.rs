use super::preferences::PreferencesPlugins;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContexts;

use crate::{input::ExtendedButtonInput, rich_text};

const MENU_WITDTH: f32 = 200.0;

#[derive(Resource)]
struct ContextMenuState {
    show_menu: bool,
    position: egui::Pos2,
    show_submenu: Submenu,
}

#[derive(Default, PartialEq)]
enum Submenu {
    #[default]
    None,
    Preferences,
}

impl Default for ContextMenuState {
    fn default() -> Self {
        Self {
            show_menu: false,
            position: egui::Pos2::ZERO,
            show_submenu: Submenu::default(),
        }
    }
}

pub struct QuickMenuPlugins;

impl Plugin for QuickMenuPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<ContextMenuState>()
            .add_systems(Update, (right_click_system, ui_context_menu_system))
            .add_plugins(PreferencesPlugins);
    }
}

fn right_click_system(
    mut context_menu_state: ResMut<ContextMenuState>,
    mouse_button_input: Res<ExtendedButtonInput>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if mouse_button_input.clicked(MouseButton::Right) {
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

    let mut style = (*ctx.style()).clone();
    style.spacing.button_padding = egui::vec2(8.0, 4.0); // Adjust padding
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_gray(200); // Hover color

    ctx.set_style(style);

    if context_menu_state.show_menu {
        let mut preferences_button_rect = None;


        egui::Window::new("Context Menu")
            .fixed_pos(context_menu_state.position)
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .default_width(MENU_WITDTH)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                   let preferences_button = ui.selectable_label(
                       context_menu_state.show_submenu == Submenu::Preferences,
                       rich_text!("Preferences"));
                   if preferences_button.clicked() {
                       context_menu_state.show_submenu = Submenu::Preferences;
                   }
                   // Store the rect of the "Preferences" button
                   preferences_button_rect = Some(preferences_button.rect);

                   if ui.selectable_label(false, rich_text!("Close"))
                       .clicked() {
                       context_menu_state.show_menu = false;
                   }
                });
            });

        // Display the second context menu if needed
        match context_menu_state.show_submenu { 
            Submenu::Preferences =>
            if let Some(preferences_rect) = preferences_button_rect {
                // Calculate the position for the submenu
                let submenu_position = context_menu_state.position
                    + egui::vec2(preferences_rect.width(), preferences_rect.min.y - context_menu_state.position.y);

                egui::Window::new("Preferences Menu")
                    .fixed_pos(submenu_position)
                    .collapsible(false)
                    .resizable(false)
                    .title_bar(false)
                    .show(ctx, |ui| {
                        ui.label("Preference 1");
                        ui.label("Preference 2");
                        if ui.button("Back").clicked() {
                            context_menu_state.show_submenu = Submenu::None;
                        }
                    });
            },
            Submenu::None => {}
        }

        // Close the context menus if clicked elsewhere
        if ctx.input(|i| i.pointer.any_down()) && !ctx.is_pointer_over_area() {
            context_menu_state.show_menu = false;
            context_menu_state.show_submenu = Submenu::None;
        }
    }
}

