use std::marker::PhantomData;

use super::preferences::PreferencesPlugins;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContexts;

use crate::{input::ExtendedButtonInput, rich_text};

pub const DEFAULT_MENU_INNER_WIDTH: f32 = 200.0;
pub const DEFAULT_MARGIN: egui::Margin = egui::Margin {
    left: 6.,
    right: 6.,
    top: 6.,
    bottom: 6.,
};
pub const DEFAULT_MENU_WIDTH: f32 = 200.0 + DEFAULT_MARGIN.left + DEFAULT_MARGIN.right;

#[derive(Default, Clone, Copy)]
pub enum Side {
    Left,
    #[default]
    Right,
}

#[derive(Resource)]
pub struct SubContmenu<T> {
    _marker: PhantomData<T>,
    rect: Option<egui::Rect>,
    /// Side where sub context menu appiar
    /// default [Side::Right], if space on screan not enought 
    /// then [Side::Left]
    appeared: Side
}

impl<T> Default for SubContmenu<T> {
    fn default() -> Self {
        Self {
            _marker: PhantomData::default(),
            rect: None,
            appeared: Side::default(),
        }
    }
}

impl<T> SubContmenu<T> {
    pub fn side(&self) -> Side {
        self.appeared
    }

    pub fn appeared(&self, side: Side) -> Side {
        self.appeared
    }

    pub fn rect(&self) -> egui::Rect {
        self.rect.unwrap()
    }

    pub fn get_rect(&self) -> Option<egui::Rect> {
        self.rect
    }

    pub fn set_rect(&mut self, rect: egui::Rect) {
        self.rect = Some(rect);
    }

    pub fn width(&self) -> f32 {
        if let Some(rect) = self.get_rect() {
            rect.width()
        } else {
            DEFAULT_MENU_WIDTH
        }
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum ContmenuSubmenu {
    #[default]
    Closed,
    Preferences,
}

#[derive(Resource)]
pub struct Contmenu {
    show_menu: bool,
    position: egui::Pos2,
    rect: Option<egui::Rect>,
}

impl Contmenu {
    pub fn rect(&self) -> egui::Rect {
        self.rect.unwrap()
    }

    pub fn get_rect(&self) -> Option<egui::Rect> {
        self.rect
    }
}

impl Default for Contmenu {
    fn default() -> Self {
        Self {
            show_menu: false,
            position: egui::Pos2::ZERO,
            rect: None,
        }
    }
}

pub struct ContmenuPlugins;

impl Plugin for ContmenuPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<Contmenu>()
            .insert_state(ContmenuSubmenu::default())
            .add_systems(Startup, setup_styles)
            .add_systems(Update, (right_click_system, ui_context_menu_system))
            .add_plugins(PreferencesPlugins);
    }
}

pub fn contmenu_window<'open>(
    title: impl Into<egui::WidgetText>,
    position: impl Into<egui::Pos2>,
) -> egui::Window<'open> {
    egui::Window::new(title)
        .fixed_pos(position)
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .default_width(DEFAULT_MENU_INNER_WIDTH)
}

fn right_click_system(
    mut context_menu: ResMut<Contmenu>,
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
    style.spacing.window_margin = DEFAULT_MARGIN;
    //style.visuals.selection.bg_fill = egui::Color32::from_gray(200);
    style.visuals.selection.stroke.width = 0.;

    ctx.set_style(style);
    //ctx.set_debug_on_hover(true);
}

fn ui_context_menu_system(
    mut contexts: EguiContexts,
    mut context_menu: ResMut<Contmenu>,
    submenu_state: Res<State<ContmenuSubmenu>>,
    mut next_submenu_state: ResMut<NextState<ContmenuSubmenu>>,
) {
    let ctx = contexts.ctx_mut();

    use ContmenuSubmenu::*;

    if context_menu.show_menu {
        let mut preferences_button_rect = None;

        let window_response = contmenu_window("Context Menu", context_menu.position)
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
            next_submenu_state.set(ContmenuSubmenu::Closed);
        }
    }
}
