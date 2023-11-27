// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use audio::{MusicChannel, SoundChannel, VolumeSettings};
use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioPlugin};
use clouds::CloudMaterial;
use equipment::EquipmentPlugin;
use player::PlayerPlugin;
use post_process::PostProcessPlugin;
use states::{level::LevelPlugin, loading::LoadingPlugin, menu::MenuPlugin};
use ui::{constants::SKY_BLUE, UiPlugin};

#[cfg(debug_assertions)]
use bevy_editor_pls::EditorPlugin;

mod audio;
mod camera;
mod cave;
mod clouds;
mod equipment;
mod level_manager;
mod map;
mod player;
mod post_process;
mod scale;
mod states;
mod ui;
mod undo;
mod util;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            AudioPlugin,
            MaterialPlugin::<CloudMaterial>::default(),
            LoadingPlugin,
            MenuPlugin,
            LevelPlugin,
            PlayerPlugin,
            EquipmentPlugin,
            UiPlugin,
            PostProcessPlugin,
            #[cfg(debug_assertions)]
            EditorPlugin::default(),
        ))
        .add_state::<states::GameState>()
        .insert_resource(ClearColor(SKY_BLUE))
        .insert_resource(AmbientLight {
            brightness: 1.0,
            ..Default::default()
        })
        .insert_resource(Msaa::Sample4)
        .add_audio_channel::<MusicChannel>()
        .add_audio_channel::<SoundChannel>()
        .insert_resource(VolumeSettings::default())
        .run();
}
