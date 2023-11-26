use std::time::Duration;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_kira_audio::{AudioChannel, AudioControl, AudioTween};

use crate::{
    audio::{AudioAssets, MusicChannel, SoundChannel, VolumeSettings},
    camera::{camera_spin, MainCamera},
    clouds::CloudMaterial,
    level_manager::init_level_manager,
    post_process::TransitionSettings,
    ui::{
        constants::{UI_YELLOW, UI_YELLOW_HOVER},
        UiRoot,
    },
};

use super::{
    level::{animate_flag, DespawnOnTransition},
    loading::{FontAssets, ModelAssets},
    transition::TransitionManager,
    GameState,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::MainMenu),
            (setup_menu, init_level_manager),
        )
        .add_systems(
            Update,
            (
                button_system,
                update_button_volume_text,
                camera_spin,
                animate_flag,
            )
                .run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(OnExit(GameState::MainMenu), cleanup_menu);
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cloud_materials: ResMut<Assets<CloudMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    font_assets: Res<FontAssets>,
    model_assets: Res<ModelAssets>,
    audio_assets: Res<AudioAssets>,
    music_channel: Res<AudioChannel<MusicChannel>>,
) {
    music_channel
        .play(audio_assets.bgm.clone())
        .looped()
        .fade_in(AudioTween::linear(Duration::from_secs(3)));

    // draw UI
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
        .insert(UiRoot)
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

    // spawn mountain
    let grid_heights = vec![
        vec![1, 3, 4, 2, 2, 1],
        vec![2, 5, 6, 7, 5, 2],
        vec![3, 6, 8, 7, 6, 4],
        vec![2, 4, 6, 5, 3, 2],
        vec![1, 2, 4, 3, 2, 1],
    ];
    for y in 0..grid_heights.len() {
        for x in 0..grid_heights[0].len() {
            let colour = Color::rgb(0.353, 0.376, 0.529);
            commands
                .spawn(MaterialMeshBundle {
                    material: materials.add(StandardMaterial {
                        base_color: colour,
                        metallic: 0.,
                        reflectance: 0.,
                        perceptual_roughness: 1.0,
                        ..Default::default()
                    }),
                    mesh: meshes.add(shape::Box::new(1.0, grid_heights[y][x] as f32, 1.0).into()),
                    transform: Transform::from_xyz(
                        x as f32,
                        grid_heights[y][x] as f32 / 2.0,
                        y as f32,
                    ),
                    ..Default::default()
                })
                .insert(DespawnOnTransition);
        }
    }
    // Spawn orthographic camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(3.0, 10.0, 10.0)
                .looking_at(Vec3::new(3.0, 5.5, 0.0), Vec3::Y),
            projection: Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(14.),
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(MainCamera::default())
        .insert(TransitionSettings::default());

    // Spawn main light source
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..Default::default()
            },
            transform: Transform::from_xyz(10.0, 20.0, 5.0).looking_at(Vec3::default(), Vec3::Y),
            ..Default::default()
        })
        .insert(DespawnOnTransition);

    // Spawn lower clouds
    commands
        .spawn(MaterialMeshBundle {
            material: cloud_materials.add(CloudMaterial {
                color_a: Color::rgba(0.7, 0.7, 0.7, 1.0),
                color_b: Color::WHITE,
            }),
            mesh: meshes.add(
                shape::Plane {
                    size: 25.0,
                    subdivisions: 18,
                }
                .into(),
            ),
            transform: Transform::from_xyz(2.5, 0., 3.0),
            ..Default::default()
        })
        .insert(Name::new("Clouds"))
        .insert(DespawnOnTransition);

    // Spawn flagpole
    commands
        .spawn(SceneBundle {
            scene: model_assets.flag.clone(),
            ..Default::default()
        })
        .insert(Transform::from_xyz(2.0, grid_heights[2][2] as f32, 2.0))
        .insert(Name::new("Flag"))
        .insert(DespawnOnTransition);
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
