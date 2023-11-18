use bevy::{prelude::*, render::camera::ScalingMode};

use crate::{
    camera::{camera_rotation, MainCamera},
    clouds::CloudMaterial,
    equipment::Inventory,
    map::{create_map_on_level_load, Map},
    player::clear_player_history,
    scale::{scale_rotation, Scale},
    ui::{
        equipment::{
            draw_equimpment_cards, draw_inventory_icons, handle_add_buttons,
            handle_subtract_buttons, update_inventory_counters,
        },
        stamina::{setup_stamina_ui, update_stamina_ui},
    },
    util::CardinalDirection,
};

use super::{loading::ModelAssets, transition::TransitionManager, GameState};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LevelManager>()
            .add_systems(
                OnEnter(GameState::Level),
                (
                    create_map_on_level_load,
                    setup_scene,
                    setup_stamina_ui,
                    draw_equimpment_cards,
                    draw_inventory_icons,
                ),
            )
            .add_systems(
                Update,
                (
                    animate_flag,
                    update_stamina_ui,
                    handle_add_buttons,
                    handle_subtract_buttons,
                    reload_level,
                    skip_level,
                    scale_rotation,
                    camera_rotation,
                )
                    .run_if(in_state(GameState::Level)),
            )
            .add_systems(
                Update,
                update_inventory_counters.run_if(resource_changed::<Inventory>()),
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
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct LevelManager {
    pub levels: Vec<Level>,
    pub current: usize,
}
impl LevelManager {
    pub fn get_current_level(&self) -> &Level {
        &self.levels[self.current]
    }
    pub fn get_current_map_mut(&mut self) -> &mut Map {
        &mut self.levels[self.current].map
    }
}

pub fn init_level_manager(mut commands: Commands) {
    commands.insert_resource(LevelManager {
        current: 0,
        levels: vec![
            Level {
                map: Map::new(
                    vec![
                        vec![5, 6, 7, 6],
                        vec![4, 4, 3, 5],
                        vec![4, 3, 3, 4],
                        vec![4, 2, 2, 3],
                        vec![1, 1, 1, 2],
                    ],
                    vec![
                        vec![false, false, false, false],
                        vec![false, false, false, false],
                        vec![false, true, false, false],
                        vec![false, false, true, false],
                        vec![true, true, true, false],
                    ],
                    (0, 4),
                    (2, 1),
                    None,
                ),
                stamina_budget: 13,
                weight_budget: 0,
                ladder_unlocked: false,
                rope_unlocked: false,
            },
            Level {
                map: Map::new(
                    vec![
                        vec![4, 6, 4, 6, 6, 4],
                        vec![4, 6, 2, 6, 5, 5],
                        vec![3, 3, 1, 3, 3, 2],
                    ],
                    vec![
                        vec![false, false, false, false, false, false],
                        vec![false, false, false, false, false, false],
                        vec![false, false, false, false, false, false],
                    ],
                    (0, 2),
                    (5, 1),
                    None,
                ),
                stamina_budget: 8,
                weight_budget: 4,
                ladder_unlocked: true,
                rope_unlocked: false,
            },
            Level {
                map: Map::new(
                    vec![
                        vec![6, 6, 6, 6, 5, 5],
                        vec![4, 4, 3, 4, 3, 2],
                        vec![2, 3, 2, 4, 2, 2],
                    ],
                    vec![
                        vec![false, false, false, false, false, false],
                        vec![false, false, false, false, false, false],
                        vec![false, false, false, false, false, false],
                    ],
                    (0, 0),
                    (5, 2),
                    None,
                ),
                stamina_budget: 9,
                weight_budget: 2,
                ladder_unlocked: false,
                rope_unlocked: true,
            },
            Level {
                map: Map::new(
                    vec![
                        vec![7, 7, 7, 6],
                        vec![6, 5, 5, 5],
                        vec![3, 3, 3, 3],
                        vec![2, 1, 1, 1],
                    ],
                    vec![
                        vec![true, true, true, true],
                        vec![true, true, true, true],
                        vec![true, true, true, true],
                        vec![true, true, true, true],
                    ],
                    (1, 3),
                    (2, 0),
                    Some((0, 0)),
                ),
                stamina_budget: 19,
                weight_budget: 3,
                ladder_unlocked: true,
                rope_unlocked: true,
            },
            Level {
                map: Map::new(
                    vec![
                        vec![5, 6, 6, 5, 6, 4],
                        vec![2, 3, 3, 2, 3, 3],
                        vec![1, 3, 1, 2, 3, 1],
                    ],
                    vec![
                        vec![true, true, true, true, true, true],
                        vec![true, true, true, true, true, true],
                        vec![true, true, true, true, true, true],
                    ],
                    (0, 2),
                    (5, 0),
                    None,
                ),
                stamina_budget: 30,
                weight_budget: 5,
                ladder_unlocked: true,
                rope_unlocked: true,
            },
        ],
    })
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
            Transform::from_xyz(1.5, 7., 10.0).looking_at(Vec3::new(2.5, 2.0, 0.0), Vec3::Y);
        *camera_projection = Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(12.),
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
                scale: 2.0,
                color_a: Color::rgb(0.7, 0.7, 0.7),
                color_b: Color::WHITE,
                height_scale: 1.0,
                time_scale: 0.2,
            }),
            mesh: meshes.add(
                shape::Plane {
                    size: 20.0,
                    subdivisions: 16,
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
        let height = map.grid_heights[scale_y as usize][scale_x as usize] as f32 + 0.3;
        let mut transform = Transform::from_xyz(scale_x as f32, height, scale_y as f32);
        transform.rotate_local_x(-0.2);
        commands
            .spawn(SceneBundle {
                scene: model_assets.scale.clone(),
                transform,
                ..Default::default()
            })
            .insert(Scale(height))
            .insert(Name::new("Scale"))
            .insert(DespawnOnTransition);
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
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        // remove ladders/ropes etc.
        level_manager.get_current_map_mut().reset();
        // reset camera rotation
        for mut camera in main_camera.iter_mut() {
            camera.direction = CardinalDirection::North;
            camera.angle_change = 0.0;
        }
        *transition_manager = TransitionManager::TransitioningOutReload(0.0);
    }
}

fn skip_level(
    mut transition_manager: ResMut<TransitionManager>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        *transition_manager = TransitionManager::TransitioningOut(0.0);
    }
}
