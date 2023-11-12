use bevy::{prelude::*, render::camera::ScalingMode};

use crate::{
    clouds::CloudMaterial,
    equipment::Inventory,
    map::{create_map_on_level_load, Map},
    ui::{
        equipment::{
            draw_equimpment_cards, handle_add_buttons, handle_subtract_buttons,
            update_inventory_counters,
        },
        stamina::{setup_stamina_ui, update_stamina_ui},
    },
};

use super::{loading::ModelAssets, GameState};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Level),
            (
                create_map_on_level_load,
                setup_scene,
                setup_stamina_ui,
                draw_equimpment_cards,
            ),
        )
        .add_systems(
            Update,
            (
                animate_flag,
                update_stamina_ui,
                handle_add_buttons,
                handle_subtract_buttons,
            )
                .run_if(in_state(GameState::Level)),
        )
        .add_systems(
            Update,
            update_inventory_counters.run_if(resource_changed::<Inventory>()),
        );
    }
}

#[derive(Debug)]
pub struct Level {
    pub map: Map,
    pub stamina_budget: u16,
}

#[derive(Debug, Resource)]
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
                        vec![8, 7, 7, 6],
                        vec![6, 5, 5, 5],
                        vec![3, 3, 3, 3],
                        vec![2, 1, 1, 1],
                    ],
                    (1, 3),
                    (2, 0),
                ),
                stamina_budget: 17,
            },
            Level {
                map: Map::new(
                    vec![
                        vec![5, 6, 6, 5, 6, 4],
                        vec![2, 3, 3, 2, 3, 3],
                        vec![1, 3, 1, 2, 3, 1],
                    ],
                    (0, 2),
                    (5, 0),
                ),
                stamina_budget: 30,
            },
        ],
    })
}

fn setup_scene(
    mut commands: Commands,
    mut cloud_materials: ResMut<Assets<CloudMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    model_assets: Res<ModelAssets>,
    level_manager: Res<LevelManager>,
) {
    // Spawn orthographic camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(1.5, 7., 10.0).looking_at(Vec3::new(2.5, 2.0, 0.0), Vec3::Y),
        projection: Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(12.),
            ..Default::default()
        }),
        ..Default::default()
    });
    // .insert(ScreenSpaceAmbientOcclusionBundle::default());

    // Spawn main light source
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(10.0, 20.0, 5.0).looking_at(Vec3::default(), Vec3::Y),
        ..Default::default()
    });

    // Spawn lower clouds
    commands.spawn(MaterialMeshBundle {
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
    });

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
        .insert(Name::new("Flag"));
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
