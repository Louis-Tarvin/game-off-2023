// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_kira_audio::AudioPlugin;
use clouds::CloudMaterial;
use equipment::EquipmentPlugin;
use player::PlayerPlugin;
use states::{level::LevelPlugin, loading::LoadingPlugin, menu::MenuPlugin};

mod clouds;
mod equipment;
mod map;
mod player;
mod states;
mod util;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            AudioPlugin,
            EditorPlugin::default(),
            MaterialPlugin::<CloudMaterial>::default(),
            LoadingPlugin,
            MenuPlugin,
            LevelPlugin,
            PlayerPlugin,
            EquipmentPlugin,
        ))
        .add_state::<states::GameState>()
        .insert_resource(ClearColor(Color::rgb(0.447, 0.867, 0.969)))
        .insert_resource(AmbientLight {
            brightness: 1.0,
            ..Default::default()
        })
        .insert_resource(Msaa::Sample4)
        .run();
}
