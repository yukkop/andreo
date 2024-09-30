use crate::{
    rich_text,
    ui::quickmenu::{ContextMenu, ContextMenuSubmenu, MARGIN, MENU_WITDTH},
};
use bevy::prelude::*;
use bevy_egui::EguiContexts;

use super::camera_controll::CameraMovementPlugin;

#[derive(Resource)]
pub struct PreferencesMenu {
    position: egui::Pos2,
    rect: Option<egui::Rect>,
}

impl Default for PreferencesMenu {
    fn default() -> Self {
        Self {
            position: egui::Pos2::ZERO,
            rect: None,
        }
    }
}

impl PreferencesMenu {
    pub fn rect(&self) -> egui::Rect {
        self.rect.unwrap()
    }

    pub fn get_rect(&self) -> Option<egui::Rect> {
        self.rect
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum PreferencesSubmenu {
    #[default]
    Closed,
    CameraContoll,
}

pub struct PreferencesPlugins;

impl Plugin for PreferencesPlugins {
    fn build(&self, app: &mut App) {
        app
            .insert_state(PreferencesSubmenu::default())
            .init_resource::<PreferencesMenu>()
            .add_systems(
              Update,
              ui_context_menu_system.run_if(in_state(ContextMenuSubmenu::Preferences)),
            )
            .add_plugins(CameraMovementPlugin);
    }
}
fn ui_context_menu_system(
    mut contexts: EguiContexts,
    mut context_menu: ResMut<ContextMenu>,
    mut next_contextmenu_submenu_state: ResMut<NextState<ContextMenuSubmenu>>,
    mut preferences_menu: ResMut<PreferencesMenu>,
    mut preferences_submenu_state: ResMut<State<PreferencesSubmenu>>,
    mut next_preferences_submenu_state: ResMut<NextState<PreferencesSubmenu>>,
) {
    let ctx = contexts.ctx_mut();

    // Get the screen rect
    let context_menu_rect = context_menu.rect();
    let context_menu_width = context_menu_rect.width();
    let context_menu_min_y = context_menu_rect.min.y;
    let screen_rect = ctx.input(|i| i.screen_rect);
    let submenu_width = MENU_WITDTH; // Assuming submenu width is MENU_WITDTH
    let mut submenu_position = context_menu_rect.min
        + egui::vec2(
            context_menu_width,
            context_menu_min_y - context_menu_rect.min.y,
        );

    // Check if the submenu would go off-screen to the right
    if submenu_position.x + submenu_width > screen_rect.max.x {
        // Not enough space on the right, so place it to the left
        submenu_position =
            context_menu_rect.min - egui::vec2(submenu_width + MARGIN.left + MARGIN.right, 0.0);

        // Ensure the submenu does not go off-screen to the left
        if submenu_position.x < screen_rect.min.x {
            submenu_position.x = screen_rect.min.x;
        }
    }

    use PreferencesSubmenu::*;

    let preferences_responce = egui::Window::new("Preferences Menu")
        .fixed_pos(submenu_position)
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .default_width(submenu_width)
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                let preferences_button = 
                    ui.selectable_label(
                        *preferences_submenu_state == CameraContoll,
                        rich_text!("Camera Controll"));
                if preferences_button.hovered() {
                    next_preferences_submenu_state.set(CameraContoll);
                } 
            });
        });

    if let Some(preferences_rect) = preferences_responce {
        preferences_menu.rect = Some(preferences_rect.response.rect);
    } else {
        log::error!("window rect not found");
    }

    if ctx.input(|i| i.pointer.any_down()) && !ctx.is_pointer_over_area() {
        next_contextmenu_submenu_state.set(ContextMenuSubmenu::Closed);
    }
}
