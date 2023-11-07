use bevy::{prelude::*, render::camera::ScalingMode};

use crate::{
    clouds::CloudMaterial,
    map::{create_map_on_level_load, Map},
};

use super::{loading::ModelAssets, GameState};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Level),
            (create_map_on_level_load, setup_scene),
        )
        .add_systems(Update, animate_flag.run_if(in_state(GameState::Level)));
        // .add_systems(
        // Update,
        // ((
        // rotate_cube,
        // update_camera_transform,
        // moving_platform::move_platforms,
        // )
        // .run_if(in_state(GameState::Playing)),),
        // );
    }
}

fn setup_scene(
    mut commands: Commands,
    mut cloud_materials: ResMut<Assets<CloudMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    model_assets: Res<ModelAssets>,
    map: Res<Map>,
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
