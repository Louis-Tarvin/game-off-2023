use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    audio::{AudioAssets, SoundChannel},
    camera::{camera_rotation, MainCamera},
    cave::{swap_cave_visibility, HasGem},
    clouds::CloudMaterial,
    equipment::Inventory,
    level_manager::LevelManager,
    map::{create_map_on_level_load, Map},
    player::clear_player_history,
    scale::{rotation, spawn_scale, ScaleCounter},
    ui::keys::StaminaCosts,
    util::CardinalDirection,
};

use super::{loading::ModelAssets, transition::TransitionManager, GameState};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StaminaCosts::default())
            .insert_resource(ScaleCounter::default())
            .insert_resource(HasGem::default())
            .register_type::<LevelManager>()
            .add_systems(
                OnEnter(GameState::Level),
                (create_map_on_level_load, setup_scene),
            )
            .add_systems(
                Update,
                (
                    animate_flag,
                    reload_level,
                    skip_level,
                    rotation,
                    camera_rotation,
                    swap_cave_visibility.run_if(resource_changed::<HasGem>()),
                )
                    .run_if(in_state(GameState::Level)),
            )
            .add_systems(
                OnEnter(GameState::LevelTransition),
                (level_transition, clear_player_history),
            )
            .add_systems(
                OnEnter(GameState::LevelReload),
                (level_transition, clear_player_history),
            );
    }
}

#[derive(Debug, Reflect)]
pub struct Level {
    pub map: Map,
    pub stamina_budget: u16,
    pub weight_budget: u8,
    pub ladder_unlocked: bool,
    pub rope_unlocked: bool,
    pub potion_unlocked: bool,
    pub rewind_unlocked: bool,
}

#[derive(Component)]
pub struct DespawnOnTransition;

fn setup_scene(
    mut commands: Commands,
    mut cloud_materials: ResMut<Assets<CloudMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cameras: Query<(&mut Transform, &mut Projection), With<MainCamera>>,
    model_assets: Res<ModelAssets>,
    level_manager: Res<LevelManager>,
) {
    // Spawn orthographic camera
    if let Ok((mut camera_transform, mut camera_projection)) = cameras.get_single_mut() {
        *camera_transform =
            Transform::from_xyz(3.5, 8.5, 10.0).looking_at(Vec3::new(4.5, 3.5, 0.0), Vec3::Y);
        *camera_projection = Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(10.),
            ..Default::default()
        });
    }

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
    let map = &level_manager.get_current_level().map;
    commands
        .spawn(SceneBundle {
            scene: model_assets.flag.clone(),
            ..Default::default()
        })
        .insert(Transform::from_xyz(
            map.flag_pos.0 as f32,
            map.grid_heights[map.flag_pos.1 as usize][map.flag_pos.0 as usize] as f32,
            map.flag_pos.1 as f32,
        ))
        .insert(Name::new("Flag"))
        .insert(DespawnOnTransition);

    // Spawn scale
    if let Some((scale_x, scale_y)) = map.scale_pos {
        spawn_scale(
            &mut commands,
            scale_x,
            scale_y,
            map.grid_heights[scale_y as usize][scale_x as usize],
            model_assets.scale.clone(),
        );
    }
}

fn animate_flag(
    model_assets: Res<ModelAssets>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player
            .play(model_assets.flag_animation.clone_weak())
            .repeat();
    }
}

fn level_transition(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
    entities: Query<Entity, With<DespawnOnTransition>>,
    mut level_manager: ResMut<LevelManager>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.insert_resource(Inventory::default());
    if matches!(current_state.get(), GameState::LevelTransition)
        && level_manager.current + 1 < level_manager.levels.len()
    {
        level_manager.current += 1;
    }
    level_manager.get_current_map_mut().reset();
    next_state.set(GameState::Level);
}

fn reload_level(
    mut transition_manager: ResMut<TransitionManager>,
    keyboard_input: Res<Input<KeyCode>>,
    mut level_manager: ResMut<LevelManager>,
    mut main_camera: Query<&mut MainCamera>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        // remove ladders/ropes etc.
        level_manager.get_current_map_mut().reset();
        // reset camera rotation
        for mut camera in main_camera.iter_mut() {
            camera.direction = CardinalDirection::North;
            camera.angle_change = 0.0;
        }
        sound_channel.play(audio_assets.woosh.clone());
        *transition_manager = TransitionManager::TransitioningOutReload(0.0);
    }
}

fn skip_level(
    mut transition_manager: ResMut<TransitionManager>,
    keyboard_input: Res<Input<KeyCode>>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        sound_channel.play(audio_assets.woosh.clone());
        *transition_manager = TransitionManager::TransitioningOut(0.0);
    }
}
