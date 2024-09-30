use super::preferences::PreferencesPlugins;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContexts;

use crate::{input::ExtendedButtonInput, rich_text};

pub const MENU_WITDTH: f32 = 200.0;
pub const MARGIN: egui::Margin = egui::Margin {
    left: 6.,
    right: 6.,
    top: 6.,
    bottom: 6.,
};

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum ContextMenuSubmenu {
    #[default]
    Closed,
    Preferences,
}

#[derive(Resource)]
pub struct ContextMenu {
    show_menu: bool,
    position: egui::Pos2,
    rect: Option<egui::Rect>,
}

impl ContextMenu {
    pub fn rect(&self) -> egui::Rect {
        self.rect.unwrap()
    }

    pub fn get_rect(&self) -> Option<egui::Rect> {
        self.rect
    }
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self {
            show_menu: false,
            position: egui::Pos2::ZERO,
            rect: None,
        }
    }
}

pub struct QuickMenuPlugins;

impl Plugin for QuickMenuPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<ContextMenu>()
            .insert_state(ContextMenuSubmenu::default())
            .add_systems(Startup, setup_styles)
            .add_systems(Update, (right_click_system, ui_context_menu_system))
            .add_plugins(PreferencesPlugins);
    }
}

fn right_click_system(
    mut context_menu: ResMut<ContextMenu>,
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

            context_menu.show_menu = true;
            context_menu.position = egui_position;
        }
    }
}

fn setup_styles(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    let mut style = (*ctx.style()).clone();
    style.spacing.button_padding = egui::vec2(8.0, 4.0); // Adjust padding
    style.spacing.window_margin = MARGIN;
    //style.visuals.selection.bg_fill = egui::Color32::from_gray(200); 
    style.visuals.selection.stroke.width = 0.; 

    ctx.set_style(style);
}

fn ui_context_menu_system(
    mut contexts: EguiContexts,
    mut context_menu: ResMut<ContextMenu>,
    submenu_state: Res<State<ContextMenuSubmenu>>,
    mut next_submenu_state: ResMut<NextState<ContextMenuSubmenu>>,
) {
    let ctx = contexts.ctx_mut();

    use ContextMenuSubmenu::*;

    if context_menu.show_menu {
        let mut preferences_button_rect = None;

        let window_response = egui::Window::new("Context Menu")
            .fixed_pos(context_menu.position)
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .default_width(MENU_WITDTH)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    let preferences_button = ui
                        .selectable_label(*submenu_state == Preferences, rich_text!("Preferences"));
                    if preferences_button.hovered() {
                        next_submenu_state.set(Preferences);
                    } 
                    // Store the rect of the "Preferences" button
                    preferences_button_rect = Some(preferences_button.rect);
                });
            });

        if let Some(window_rect) = window_response {
            context_menu.rect = Some(window_rect.response.rect);
        } else {
            log::error!("window rect not found");
        }

        // Close the context menus if clicked elsewhere
        if ctx.input(|i| i.pointer.any_down()) && !ctx.is_pointer_over_area() {
            context_menu.show_menu = false;
            next_submenu_state.set(ContextMenuSubmenu::Closed);
        }
    }
}
