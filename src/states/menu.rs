use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl, AudioTween};

use crate::{
    audio::{AudioAssets, MusicChannel, SoundChannel, VolumeSettings},
    camera::MainCamera,
    level_manager::init_level_manager,
    post_process::TransitionSettings,
    ui::constants::{UI_YELLOW, UI_YELLOW_HOVER},
};

use super::{
    loading::FontAssets,
    transition::{update_transition_manager, TransitionManager},
    GameState,
};

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::MainMenu),
            (setup_menu, init_level_manager),
        )
        .add_systems(
            Update,
            (button_system, update_button_volume_text).run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(OnExit(GameState::MainMenu), cleanup_menu)
        .add_systems(
            Update,
            update_transition_manager
                .run_if(in_state(GameState::MainMenu).or_else(in_state(GameState::Level))),
        );
    }
}

#[derive(Component)]
enum MenuButton {
    Start,
    Sound,
    Music,
}

#[derive(Component)]
struct MainMenuRoot;

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    audio_assets: Res<AudioAssets>,
    music_channel: Res<AudioChannel<MusicChannel>>,
) {
    music_channel
        .play(audio_assets.bgm.clone())
        .looped()
        .fade_in(AudioTween::linear(Duration::from_secs(3)));

    commands
        .spawn(Camera3dBundle::default())
        .insert(MainCamera::default())
        .insert(TransitionSettings { progress: 0.0 });
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainMenuRoot)
        .with_children(|parent| {
            add_button(
                parent,
                "Start",
                MenuButton::Start,
                font_assets.fira_sans.clone(),
            );
            add_button(
                parent,
                "Sound",
                MenuButton::Sound,
                font_assets.fira_sans.clone(),
            );
            add_button(
                parent,
                "Music",
                MenuButton::Music,
                font_assets.fira_sans.clone(),
            );
        });
}

fn add_button(parent: &mut ChildBuilder, text: &str, button: MenuButton, font: Handle<Font>) {
    let button_style = Style {
        width: Val::Px(220.0),
        height: Val::Px(60.0),
        // center button
        margin: UiRect::all(Val::Px(20.0)),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..Default::default()
    };
    parent
        .spawn(ButtonBundle {
            style: button_style,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));
        })
        .insert(button);
}

fn button_system(
    mut interaction_query: Query<
        (&MenuButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut volume_settings: ResMut<VolumeSettings>,
    mut transition_manager: ResMut<TransitionManager>,
    music_channel: Res<AudioChannel<MusicChannel>>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    for (button, interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = UI_YELLOW.into();
                match button {
                    MenuButton::Start => {
                        sound_channel.play(audio_assets.woosh.clone());
                        *transition_manager = TransitionManager::TransitioningOutReload(0.0);
                    }
                    MenuButton::Sound => {
                        volume_settings.toggle_sfx_vol();
                        sound_channel.set_volume(volume_settings.sfx_vol);
                        sound_channel.play(audio_assets.pop.clone());
                    }
                    MenuButton::Music => {
                        volume_settings.toggle_music_vol();
                        music_channel.set_volume(volume_settings.music_vol);
                        sound_channel.play(audio_assets.pop.clone());
                    }
                }
            }
            Interaction::Hovered => {
                *color = UI_YELLOW_HOVER.into();
            }
            Interaction::None => {
                *color = UI_YELLOW.into();
            }
        }
    }
}

fn update_button_volume_text(
    query: Query<(&MenuButton, &Children)>,
    mut text_query: Query<&mut Text>,
    volume_settings: Res<VolumeSettings>,
) {
    if !volume_settings.is_changed() {
        return;
    }
    for (button, children) in query.iter() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match button {
            MenuButton::Sound => {
                text.sections[0].value =
                    format!(" Sound: {}% ", (volume_settings.sfx_vol * 100.0).round());
            }
            MenuButton::Music => {
                text.sections[0].value =
                    format!(" Music: {}% ", (volume_settings.music_vol * 100.0).round());
            }
            _ => {}
        }
    }
}

fn cleanup_menu(mut commands: Commands, root: Query<Entity, With<MainMenuRoot>>) {
    commands.entity(root.single()).despawn_recursive();
}
