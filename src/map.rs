use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    cave::{spawn_gem, Cave, CaveData, GemCave, HasGem},
    equipment::{
        ladder::{HorizontalLadderKey, VerticalLadderKey},
        rope::RopeKey,
    },
    level_manager::LevelManager,
    states::{level::DespawnOnTransition, loading::ModelAssets},
    util::CardinalDirection,
};

#[derive(Debug, Resource, Reflect)]
pub struct Map {
    pub grid_heights: Vec<Vec<u8>>,
    pub grid_climbable: Vec<Vec<bool>>,
    pub player_start_pos: (u8, u8),
    pub flag_pos: (u8, u8),
    pub scale_pos: Option<(u8, u8)>,
    pub vertical_ladders: HashMap<VerticalLadderKey, Entity>,
    pub horizontal_ladders: HashMap<HorizontalLadderKey, Entity>,
    pub ropes: HashMap<RopeKey, Entity>,
    pub cave_data: Option<CaveData>,
}
impl Map {
    pub fn new(
        grid_heights: Vec<Vec<u8>>,
        grid_climbable: Vec<Vec<bool>>,
        player_pos: (u8, u8),
        flag_pos: (u8, u8),
        scale_pos: Option<(u8, u8)>,
        cave_data: Option<CaveData>,
    ) -> Self {
        Self {
            grid_heights,
            grid_climbable,
            player_start_pos: player_pos,
            flag_pos,
            scale_pos,
            vertical_ladders: HashMap::new(),
            horizontal_ladders: HashMap::new(),
            ropes: HashMap::new(),
            cave_data,
        }
    }
    pub fn is_ladder_or_rope(
        &self,
        x: u8,
        y: u8,
        height: u8,
        direction: CardinalDirection,
    ) -> bool {
        let (x_offset, y_offset) = match direction {
            CardinalDirection::North => (0, -1),
            CardinalDirection::East => (1, 0),
            CardinalDirection::South => (0, 1),
            CardinalDirection::West => (-1, 0),
        };
        // grid square directly in front of the player (might be out of bounds)
        let grid_facing_x: u8 = match (x as i16 + x_offset).try_into() {
            Ok(x) => x,
            Err(_) => return false,
        };
        let grid_facing_y: u8 = match (y as i16 + y_offset).try_into() {
            Ok(x) => x,
            Err(_) => return false,
        };
        self.vertical_ladders.contains_key(&VerticalLadderKey {
            x: grid_facing_x,
            y: grid_facing_y,
            height: height - 2,
            direction,
        }) || self.vertical_ladders.contains_key(&VerticalLadderKey {
            x: grid_facing_x,
            y: grid_facing_y,
            height: height - 1,
            direction,
        }) || self.horizontal_ladders.contains_key(&HorizontalLadderKey {
            x,
            y,
            height,
            alignment: direction.into(),
        }) || self.ropes.contains_key(&RopeKey {
            x: grid_facing_x,
            y: grid_facing_y,
            direction: direction.reverse(),
        })
    }

    pub fn reset(&mut self) {
        self.vertical_ladders = HashMap::new();
        self.horizontal_ladders = HashMap::new();
        self.ropes = HashMap::new();
    }

    pub fn midpoint(&self) -> (f32, f32) {
        (
            self.grid_heights[0].len() as f32 / 2.0,
            self.grid_heights.len() as f32 / 2.0,
        )
    }
}

pub fn create_map_on_level_load(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    model_assets: Res<ModelAssets>,
    level_manager: Res<LevelManager>,
    mut has_gem: ResMut<HasGem>,
) {
    let map = &level_manager.get_current_level().map;
    for y in 0..map.grid_heights.len() {
        for x in 0..map.grid_heights[0].len() {
            let colour = if map.grid_climbable[y][x] {
                Color::rgb(0.353, 0.376, 0.529)
            } else {
                Color::rgb(0.192, 0.204, 0.286)
            };
            commands
                .spawn(MaterialMeshBundle {
                    material: materials.add(StandardMaterial {
                        base_color: colour,
                        metallic: 0.,
                        reflectance: 0.,
                        perceptual_roughness: 1.0,
                        ..Default::default()
                    }),
                    mesh: meshes
                        .add(shape::Box::new(1.0, map.grid_heights[y][x] as f32, 1.0).into()),
                    transform: Transform::from_xyz(
                        x as f32,
                        map.grid_heights[y][x] as f32 / 2.0,
                        y as f32,
                    ),
                    ..Default::default()
                })
                .insert(DespawnOnTransition);
        }
    }
    if let Some(cave_data) = &map.cave_data {
        let mut no_gem_visibility = Visibility::Hidden;
        let mut yes_gem_visibility = Visibility::Hidden;
        if let Some((x, y)) = cave_data.gem_pos {
            yes_gem_visibility = Visibility::Visible;
            has_gem.0 = false;
            let height = map.grid_heights[y as usize][x as usize] as f32 + 0.3;
            spawn_gem(&mut commands, x, y, height, model_assets.gem.clone());
        } else {
            no_gem_visibility = Visibility::Visible;
            has_gem.0 = true;
        }

        let x = cave_data.first_pos.0 as usize;
        let y = cave_data.first_pos.1 as usize;
        commands
            .spawn(SceneBundle {
                scene: model_assets.cave1.clone(),
                transform: Transform::from_xyz(x as f32, map.grid_heights[y][x] as f32, y as f32)
                    .with_rotation(Quat::from_rotation_y(-1.571)),
                visibility: no_gem_visibility,
                ..Default::default()
            })
            .insert(DespawnOnTransition)
            .insert(Cave);
        commands
            .spawn(SceneBundle {
                scene: model_assets.cave2.clone(),
                transform: Transform::from_xyz(x as f32, map.grid_heights[y][x] as f32, y as f32)
                    .with_rotation(Quat::from_rotation_y(-1.571)),
                visibility: yes_gem_visibility,
                ..Default::default()
            })
            .insert(GemCave)
            .insert(DespawnOnTransition);

        let x = cave_data.second_pos.0 as usize;
        let y = cave_data.second_pos.1 as usize;
        commands
            .spawn(SceneBundle {
                scene: model_assets.cave1.clone(),
                transform: Transform::from_xyz(x as f32, map.grid_heights[y][x] as f32, y as f32)
                    .with_rotation(Quat::from_rotation_y(-1.571)),
                visibility: no_gem_visibility,
                ..Default::default()
            })
            .insert(DespawnOnTransition)
            .insert(Cave);
        commands
            .spawn(SceneBundle {
                scene: model_assets.cave2.clone(),
                transform: Transform::from_xyz(x as f32, map.grid_heights[y][x] as f32, y as f32)
                    .with_rotation(Quat::from_rotation_y(-1.571)),
                visibility: yes_gem_visibility,
                ..Default::default()
            })
            .insert(GemCave)
            .insert(DespawnOnTransition);
    }
}
