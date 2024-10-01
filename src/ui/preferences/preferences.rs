use crate::{
    rich_text,
    ui::contmenu::{
        contmenu_window, Contmenu, ContmenuSubmenu, Side, SubContmenu, DEFAULT_MENU_INNER_WIDTH, DEFAULT_MENU_WIDTH
    },
};
use bevy::prelude::*;
use bevy_egui::EguiContexts;

use super::camera_controll::CameraMovementPlugin;

pub struct PreferencesMenu;

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum PreferencesSubmenu {
    #[default]
    Closed,
    CameraContoll,
}

pub struct PreferencesPlugins;

impl Plugin for PreferencesPlugins {
    fn build(&self, app: &mut App) {
        app.insert_state(PreferencesSubmenu::default())
            .init_resource::<SubContmenu<PreferencesMenu>>()
            .add_systems(
                Update,
                ui_context_menu_system.run_if(in_state(ContmenuSubmenu::Preferences)),
            )
            .add_plugins(CameraMovementPlugin);
    }
}

fn ui_context_menu_system(
    mut contexts: EguiContexts,
    context_menu: Res<Contmenu>,
    mut next_contextmenu_submenu_state: ResMut<NextState<ContmenuSubmenu>>,
    mut preferences_menu: ResMut<SubContmenu<PreferencesMenu>>,
    preferences_submenu_state: Res<State<PreferencesSubmenu>>,
    mut next_preferences_submenu_state: ResMut<NextState<PreferencesSubmenu>>,
) {
    let ctx = contexts.ctx_mut();

    // Get the screen rect
    let context_menu_rect = context_menu.rect();
    let context_menu_width = context_menu_rect.width();
    let context_menu_min_y = context_menu_rect.min.y;
    let screen_rect = ctx.input(|i| i.screen_rect);
    let preference_menu_width = DEFAULT_MENU_WIDTH; // Assuming submenu width is MENU_WITDTH
    let mut submenu_position = context_menu_rect.min
        + egui::vec2(
            context_menu_width,
            context_menu_min_y - context_menu_rect.min.y,
        );

    // Check if the submenu would go off-screen to the right
    if submenu_position.x + preference_menu_width > screen_rect.max.x {
        // Not enough space on the right, so place it to the left
        submenu_position = context_menu_rect.min - egui::vec2(preference_menu_width, 0.0);

        // Ensure the submenu does not go off-screen to the left
        if submenu_position.x < screen_rect.min.x {
            submenu_position.x = screen_rect.min.x;
        }

        // write that this menu appeared on left
        preferences_menu.appeared(Side::Left);
    }

    use PreferencesSubmenu::*;

    let preferences_responce =
        contmenu_window("Preferences Menu", submenu_position).show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                let preferences_button = ui.selectable_label(
                    *preferences_submenu_state == CameraContoll,
                    rich_text!("Camera Controll"),
                );
                if preferences_button.hovered() {
                    next_preferences_submenu_state.set(CameraContoll);
                }
            });
        });

    if let Some(preferences_rect) = preferences_responce {
        preferences_menu.set_rect(preferences_rect.response.rect);
    } else {
        log::error!("window rect not found");
    }

    if ctx.input(|i| i.pointer.any_down()) && !ctx.is_pointer_over_area() {
        next_contextmenu_submenu_state.set(ContmenuSubmenu::Closed);
    }
}
