use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    equipment::{
        ladder::{HorizontalLadderKey, VerticalLadderKey},
        rope::RopeKey,
    },
    util::CardinalDirection,
};

#[derive(Debug, Resource)]
pub struct Map {
    pub grid_heights: Vec<Vec<u8>>,
    pub player_start_pos: (u8, u8),
    pub flag_pos: (u8, u8),
    pub vertical_ladders: HashMap<VerticalLadderKey, Entity>,
    pub horizontal_ladders: HashMap<HorizontalLadderKey, Entity>,
    pub ropes: HashMap<RopeKey, Entity>,
}
impl Map {
    pub fn new(grid_heights: Vec<Vec<u8>>, player_pos: (u8, u8), flag_pos: (u8, u8)) -> Self {
        Self {
            grid_heights,
            player_start_pos: player_pos,
            flag_pos,
            vertical_ladders: HashMap::new(),
            horizontal_ladders: HashMap::new(),
            ropes: HashMap::new(),
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
        println!("{:?}", self.vertical_ladders);
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