use std::{
    env,
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Arc,
};

use bevy::{
    app::{App, Last, Plugin, PostStartup},
    ecs::{
        event::{Event, EventReader},
        system::{Commands, Res, ResMut, Resource},
    },
    prelude::{Deref, DerefMut},
};
use serde::{self, Deserialize, Serialize};

const FILE_NAME: &'static str = "preferences";

#[allow(dead_code)]
#[derive(Debug, Resource, Default, Clone, Deref, DerefMut)]
struct AppliedPreferences(Preferences);

#[derive(Deserialize, Serialize, Debug, Resource, Clone)]
pub struct CameraControllPreferences {
    pub rotation_sensitivity: f32,
    pub pan_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub inertia_on: bool,
    pub inertia_decrement_speed: f32,
}

impl Default for CameraControllPreferences {
    fn default() -> Self {
        Self { 
          rotation_sensitivity: 0.005,
          pan_sensitivity: 0.01,
          zoom_sensitivity: 0.5,
          min_distance: 1.0,
          max_distance: 100.0,
          inertia_on: false,
          inertia_decrement_speed: 0.02,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Resource, Clone, Default)]
pub struct Preferences {
    pub camera_controll: CameraControllPreferences,
}

#[derive(Debug, Resource, Deref)]
struct PreferencesPath(Arc<PathBuf>);

#[derive(Debug, Event)]
pub struct ApplyPreferencesEvent;

#[derive(Debug, Event)]
pub struct ExemptPreferencesEvent;

pub struct PreferencesPlugin;

impl Plugin for PreferencesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppliedPreferences>()
            .add_event::<ApplyPreferencesEvent>()
            .add_event::<ExemptPreferencesEvent>()
            .add_systems(PostStartup, setup)
            .add_systems(Last, (apply_preferences, exempt_preferences));
    }
}

fn exempt_preferences(
    mut commands: Commands,
    mut event: EventReader<ExemptPreferencesEvent>,
    applied_preferences: Res<AppliedPreferences>,
) {
    for _ in event.read() {
        commands.insert_resource((**applied_preferences).clone());
    }
}

fn apply_preferences(
    mut commands: Commands,
    mut event: EventReader<ApplyPreferencesEvent>,
    preferences: Res<Preferences>,
    preferences_path: Res<PreferencesPath>,
) {
    for _ in event.read() {
        commands.insert_resource(AppliedPreferences((*preferences).clone()));

        let preferences_path = preferences_path.as_ref().as_ref();
        let mut file = OpenOptions::new()
            .write(true)
            .open(preferences_path)
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to create preferences file ({:#?}) \n error: {:#?}",
                    preferences_path, err
                )
            });
        file.write_all(serde_yaml::to_string(preferences.as_ref()).unwrap().as_bytes())
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to write to preferences file ({:#?}) \n error: {:#?}",
                    preferences_path, err
                )
            });
    }
}

fn setup(mut commands: Commands) {
    let exe_path = env::current_exe().expect("Failed to find executable path");

    let exe_dir = exe_path
        .parent()
        .expect("Failed to find executable directory");

    let yaml_path = exe_dir.join(format!("{FILE_NAME}.yaml"));
    let yml_path = exe_dir.join(format!("{FILE_NAME}.yaml"));

    let preferences = {
        if yaml_path.exists() {
            let file = File::open(&yaml_path).unwrap_or_else(|err| {
                panic!(
                    "Failed to open exist preferences file ({:#?}) \n error: {:#?}",
                    &yaml_path, err
                )
            });

            commands.insert_resource(PreferencesPath(yaml_path.clone().into()));

            serde_yaml::from_reader(file).unwrap_or_else(|err| {
                panic!(
                    "Failed to read preferences file ({:#?}) \n error: {:#?}",
                    &yaml_path, err
                )
            })
        } else if yml_path.exists() {
            let file = File::open(&yml_path).unwrap_or_else(|err| {
                panic!(
                    "Failed to open exist preferences file ({:#?}) \n error: {:#?}",
                    &yml_path, err
                )
            });

            commands.insert_resource(PreferencesPath(yml_path.clone().into()));

            serde_yaml::from_reader(&file).unwrap_or_else(|err| {
                panic!(
                    "Failed to read preferences file ({:#?}) \n error: {:#?}",
                    &yml_path, err
                )
            })
        } else {
            let mut file: File = File::create(&yaml_path).unwrap_or_else(|err| {
                panic!(
                    "Failed to create preferences file ({:#?}) \n error: {:#?}",
                    &yaml_path, err
                )
            });

            commands.insert_resource(PreferencesPath(yaml_path.clone().into()));

            let preferences = Preferences::default();
            serde_yaml::to_writer(&mut file, &preferences).unwrap_or_else(|err| {
                panic!(
                    "Failed to write to preferences file ({:#?}) \n error: {:#?}",
                    &yaml_path, err
                )
            });

            preferences
        }
    };

    commands.insert_resource(preferences);
}
