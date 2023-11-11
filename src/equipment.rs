use std::collections::hash_map::Entry;

use bevy::prelude::*;

use crate::{
    map::Map,
    player::Player,
    states::{loading::ModelAssets, GameState},
    util::{Alignment, CardinalDirection},
};

pub struct EquipmentPlugin;

impl Plugin for EquipmentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ladder>().add_systems(
            Update,
            handle_ladder_input.run_if(in_state(GameState::Level)),
        );
    }
}

#[derive(Debug, PartialEq, Eq, Reflect)]
pub enum LadderOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct VerticalLadderKey {
    pub x: u8,
    pub y: u8,
    pub height: u8,
    pub direction: CardinalDirection,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct HorizontalLadderKey {
    pub x: u8,
    pub y: u8,
    pub height: u8,
    pub alignment: Alignment,
}

#[derive(Debug, Component, Reflect)]
pub struct Ladder;

fn handle_ladder_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&Player>,
    mut map: ResMut<Map>,
    model_assets: Res<ModelAssets>,
) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        let player = player_query
            .get_single()
            .expect("There should only be one player");
        let player_height = match &player.state {
            crate::player::PlayerState::Standing(_) => {
                map.grid_heights[player.grid_pos_y as usize][player.grid_pos_x as usize]
            }
            crate::player::PlayerState::Climbing(climb_state) => climb_state.elevation,
            crate::player::PlayerState::StandingOnLadder(ladder_state) => ladder_state.elevation,
        };
        let player_direction = match &player.state {
            crate::player::PlayerState::Standing(d) => *d,
            crate::player::PlayerState::Climbing(climb_state) => climb_state.direction,
            crate::player::PlayerState::StandingOnLadder(ladder_state) => ladder_state.direction,
        };
        let (x_offset, y_offset) = match player_direction {
            CardinalDirection::North => (0, -1),
            CardinalDirection::East => (1, 0),
            CardinalDirection::South => (0, 1),
            CardinalDirection::West => (-1, 0),
        };
        // grid square directly in front of the player (might be out of bounds)
        let grid_facing_x = player.grid_pos_x as i16 + x_offset;
        let grid_facing_y = player.grid_pos_y as i16 + y_offset;

        // Check if there is a valid vertical ladder placement
        if grid_facing_x >= 0
            && grid_facing_x < map.grid_heights[0].len() as i16
            && grid_facing_y >= 0
            && grid_facing_y < map.grid_heights.len() as i16
            && map.grid_heights[grid_facing_y as usize][grid_facing_x as usize] >= player_height + 2
        {
            match map.vertical_ladders.entry(VerticalLadderKey {
                x: grid_facing_x as u8,
                y: grid_facing_y as u8,
                height: player_height,
                direction: player_direction,
            }) {
                Entry::Occupied(o) => {
                    // there is already a ladder -> pick it up
                    // TODO: add ladder to inventory
                    commands.entity(o.remove()).despawn_recursive();
                }
                Entry::Vacant(v) => {
                    // no existing ladder -> place it
                    // TODO: check if in inventory and remove it
                    let (x_offset, y_offset) = match player_direction {
                        CardinalDirection::North => (0., -0.47),
                        CardinalDirection::East => (0.47, 0.),
                        CardinalDirection::South => (0., 0.47),
                        CardinalDirection::West => (-0.47, 0.),
                    };
                    let entity = commands
                        .spawn(SceneBundle {
                            scene: model_assets.ladder.clone(),
                            transform: Transform::from_xyz(
                                player.grid_pos_x as f32 + x_offset,
                                player_height as f32 + 0.5,
                                player.grid_pos_y as f32 + y_offset,
                            )
                            .looking_to(player_direction.into(), Vec3::Y),
                            ..Default::default()
                        })
                        .insert(Ladder)
                        .insert(Name::new("Vertical Ladder"))
                        .with_children(|parent| {
                            parent.spawn(SceneBundle {
                                scene: model_assets.ladder.clone(),
                                transform: Transform::from_xyz(0., 1., 0.),
                                ..Default::default()
                            });
                        })
                        .id();
                    v.insert(entity);
                }
            }
        } else {
            // grid square after the one directly in front of the player
            let grid_facing_x_2 = player.grid_pos_x as i16 + (x_offset * 2);
            let grid_facing_y_2 = player.grid_pos_y as i16 + (y_offset * 2);

            // Check if there is a valid horizontal ladder placement
            if grid_facing_x >= 0
                && grid_facing_x < map.grid_heights[0].len() as i16
                && grid_facing_y >= 0
                && grid_facing_y < map.grid_heights.len() as i16
                && grid_facing_x_2 >= 0
                && grid_facing_x_2 < map.grid_heights[0].len() as i16
                && grid_facing_y_2 >= 0
                && grid_facing_y_2 < map.grid_heights.len() as i16
                && ((map.grid_heights[grid_facing_y as usize][grid_facing_x as usize]
                    <= player_height
                    && map.grid_heights[grid_facing_y_2 as usize][grid_facing_x_2 as usize]
                        == player_height)
                    || (map.grid_heights[grid_facing_y as usize][grid_facing_x as usize]
                        == player_height
                        && map.grid_heights[grid_facing_y_2 as usize][grid_facing_x_2 as usize]
                            <= player_height))
            {
                let key = HorizontalLadderKey {
                    x: grid_facing_x as u8,
                    y: grid_facing_y as u8,
                    height: player_height,
                    alignment: player_direction.into(),
                };
                match map.horizontal_ladders.entry(key) {
                    Entry::Occupied(o) => {
                        // there is already a ladder -> pick it up
                        // TODO: add ladder to inventory
                        commands.entity(o.remove()).despawn_recursive();
                    }
                    Entry::Vacant(v) => {
                        // no existing ladder -> place it
                        // TODO: check if in inventory and remove it
                        let (x_offset, y_offset) = match player_direction {
                            CardinalDirection::North => (0., -0.47),
                            CardinalDirection::East => (0.47, 0.),
                            CardinalDirection::South => (0., 0.47),
                            CardinalDirection::West => (-0.47, 0.),
                        };
                        let mut transform = Transform::from_xyz(
                            player.grid_pos_x as f32 + x_offset,
                            player_height as f32,
                            player.grid_pos_y as f32 + y_offset,
                        )
                        .looking_to(player_direction.into(), Vec3::Y);
                        transform.rotate_local_x(1.571);
                        let entity = commands
                            .spawn(SceneBundle {
                                scene: model_assets.ladder.clone(),
                                transform,
                                ..Default::default()
                            })
                            .insert(Ladder)
                            .with_children(|parent| {
                                parent.spawn(SceneBundle {
                                    scene: model_assets.ladder.clone(),
                                    transform: Transform::from_xyz(0., 1., 0.),
                                    ..Default::default()
                                });
                            })
                            .id();
                        v.insert(entity);
                    }
                }
            }
        }
    }
}
