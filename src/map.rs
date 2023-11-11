use std::collections::HashMap;

use bevy::prelude::*;

use crate::equipment::{HorizontalLadderKey, VerticalLadderKey};

#[derive(Debug, Resource)]
pub struct Map {
    pub grid_heights: Vec<Vec<u8>>,
    pub player_start_pos: (u8, u8),
    pub flag_pos: (u8, u8),
    pub vertical_ladders: HashMap<VerticalLadderKey, Entity>,
    pub horizontal_ladders: HashMap<HorizontalLadderKey, Entity>,
}
impl Map {
    pub fn new(grid_heights: Vec<Vec<u8>>, player_pos: (u8, u8), flag_pos: (u8, u8)) -> Self {
        Self {
            grid_heights,
            player_start_pos: player_pos,
            flag_pos,
            vertical_ladders: HashMap::new(),
            horizontal_ladders: HashMap::new(),
        }
    }
}

pub fn create_map_on_level_load(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    map: Res<Map>,
) {
    for y in 0..map.grid_heights.len() {
        for x in 0..map.grid_heights[0].len() {
            commands.spawn(MaterialMeshBundle {
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.353, 0.376, 0.529),
                    metallic: 0.,
                    reflectance: 0.,
                    perceptual_roughness: 1.0,
                    ..Default::default()
                }),
                mesh: meshes.add(shape::Box::new(1.0, map.grid_heights[y][x] as f32, 1.0).into()),
                transform: Transform::from_xyz(
                    x as f32,
                    map.grid_heights[y][x] as f32 / 2.0,
                    y as f32,
                ),
                ..Default::default()
            });
        }
    }
}
