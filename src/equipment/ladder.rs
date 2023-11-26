use std::collections::hash_map::{Entry, VacantEntry};

use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    audio::{AudioAssets, SoundChannel},
    level_manager::LevelManager,
    map::Map,
    player::{Player, PlayerHistory, PlayerHistoryEvent},
    states::{level::DespawnOnTransition, loading::ModelAssets},
    util::{Alignment, CardinalDirection},
};

use super::Inventory;

#[derive(Debug, PartialEq, Eq, Reflect)]
pub enum LadderOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Eq, Hash, Reflect, Clone)]
pub struct VerticalLadderKey {
    pub x: u8,
    pub y: u8,
    pub height: u8,
    pub direction: CardinalDirection,
}

#[derive(Debug, PartialEq, Eq, Hash, Reflect, Clone)]
pub struct HorizontalLadderKey {
    pub x: u8,
    pub y: u8,
    pub height: u8,
    pub alignment: Alignment,
}

#[derive(Debug, Component, Reflect)]
pub struct Ladder;

fn is_valid_vertical_ladder_placement(
    map: &Map,
    grid_facing_x: i16,
    grid_facing_y: i16,
    player_height: u8,
) -> bool {
    grid_facing_x >= 0
        && grid_facing_x < map.grid_heights[0].len() as i16
        && grid_facing_y >= 0
        && grid_facing_y < map.grid_heights.len() as i16
        && map.grid_heights[grid_facing_y as usize][grid_facing_x as usize] >= player_height + 2
}

fn is_valid_horizontal_ladder_placement(
    map: &Map,
    grid_facing_x: i16,
    grid_facing_y: i16,
    grid_facing_x_2: i16,
    grid_facing_y_2: i16,
    player_height: u8,
) -> bool {
    grid_facing_x >= 0
        && grid_facing_x < map.grid_heights[0].len() as i16
        && grid_facing_y >= 0
        && grid_facing_y < map.grid_heights.len() as i16
        && grid_facing_x_2 >= 0
        && grid_facing_x_2 < map.grid_heights[0].len() as i16
        && grid_facing_y_2 >= 0
        && grid_facing_y_2 < map.grid_heights.len() as i16
        && ((map.grid_heights[grid_facing_y as usize][grid_facing_x as usize] <= player_height
            && map.grid_heights[grid_facing_y_2 as usize][grid_facing_x_2 as usize]
                == player_height)
            || (map.grid_heights[grid_facing_y as usize][grid_facing_x as usize] == player_height
                && map.grid_heights[grid_facing_y_2 as usize][grid_facing_x_2 as usize]
                    <= player_height))
}

pub fn place_vertical_ladder(
    commands: &mut Commands,
    ladder_scn: Handle<Scene>,
    direction: CardinalDirection,
    x: f32,
    y: f32,
    height: f32,
    v: VacantEntry<VerticalLadderKey, Entity>,
) {
    let (x_offset, y_offset) = match direction {
        CardinalDirection::North => (0., -0.47),
        CardinalDirection::East => (0.47, 0.),
        CardinalDirection::South => (0., 0.47),
        CardinalDirection::West => (-0.47, 0.),
    };
    let entity = commands
        .spawn(SceneBundle {
            scene: ladder_scn.clone(),
            transform: Transform::from_xyz(x + x_offset, height + 0.5, y + y_offset)
                .looking_to(direction.into(), Vec3::Y),
            ..Default::default()
        })
        .insert(Ladder)
        .insert(Name::new("Vertical Ladder"))
        .insert(DespawnOnTransition)
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: ladder_scn,
                transform: Transform::from_xyz(0., 1., 0.),
                ..Default::default()
            });
        })
        .id();
    v.insert(entity);
}

pub fn place_horizontal_ladder(
    commands: &mut Commands,
    ladder_scn: Handle<Scene>,
    direction: CardinalDirection,
    x: f32,
    y: f32,
    height: f32,
    v: VacantEntry<HorizontalLadderKey, Entity>,
) {
    let (x_offset, y_offset) = match direction {
        CardinalDirection::North => (0., -0.47),
        CardinalDirection::East => (0.47, 0.),
        CardinalDirection::South => (0., 0.47),
        CardinalDirection::West => (-0.47, 0.),
    };
    let mut transform = Transform::from_xyz(x + x_offset, height, y + y_offset)
        .looking_to(direction.reverse().into(), Vec3::Y);
    transform.rotate_local_x(1.571);
    let entity = commands
        .spawn(SceneBundle {
            scene: ladder_scn.clone(),
            transform,
            ..Default::default()
        })
        .insert(Ladder)
        .insert(Name::new("Horizontal Ladder"))
        .insert(DespawnOnTransition)
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: ladder_scn,
                transform: Transform::from_xyz(0., 1., 0.),
                ..Default::default()
            });
        })
        .id();
    v.insert(entity);
}

pub fn handle_ladder_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&Player>,
    mut level_manager: ResMut<LevelManager>,
    mut inventory: ResMut<Inventory>,
    model_assets: Res<ModelAssets>,
    mut player_history: ResMut<PlayerHistory>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        let map = level_manager.get_current_map_mut();
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
        if is_valid_vertical_ladder_placement(map, grid_facing_x, grid_facing_y, player_height) {
            let key = VerticalLadderKey {
                x: grid_facing_x as u8,
                y: grid_facing_y as u8,
                height: player_height,
                direction: player_direction,
            };
            match map.vertical_ladders.entry(key.clone()) {
                Entry::Occupied(o) => {
                    // there is already a ladder -> pick it up
                    inventory.ladder_count += 1;
                    commands.entity(o.remove()).despawn_recursive();
                    player_history
                        .0
                        .push(PlayerHistoryEvent::PickUpVerticalLadder(key));
                }
                Entry::Vacant(v) => {
                    // no existing ladder -> place it
                    if inventory.ladder_count > 0 {
                        inventory.ladder_count -= 1;
                        place_vertical_ladder(
                            &mut commands,
                            model_assets.ladder.clone(),
                            player_direction,
                            player.grid_pos_x as f32,
                            player.grid_pos_y as f32,
                            player_height as f32,
                            v,
                        );
                        player_history
                            .0
                            .push(PlayerHistoryEvent::PlaceVerticalLadder(key));

                        sound_channel.play(audio_assets.pop.clone());
                    }
                }
            }
        } else {
            // grid square after the one directly in front of the player
            let grid_facing_x_2 = player.grid_pos_x as i16 + (x_offset * 2);
            let grid_facing_y_2 = player.grid_pos_y as i16 + (y_offset * 2);

            // Check if there is a valid horizontal ladder placement
            if is_valid_horizontal_ladder_placement(
                map,
                grid_facing_x,
                grid_facing_y,
                grid_facing_x_2,
                grid_facing_y_2,
                player_height,
            ) {
                let key = HorizontalLadderKey {
                    x: grid_facing_x as u8,
                    y: grid_facing_y as u8,
                    height: player_height,
                    alignment: player_direction.into(),
                };
                match map.horizontal_ladders.entry(key.clone()) {
                    Entry::Occupied(o) => {
                        // there is already a ladder -> pick it up
                        inventory.ladder_count += 1;
                        player_history
                            .0
                            .push(PlayerHistoryEvent::PickUpHorizontalLadder(key));
                        commands.entity(o.remove()).despawn_recursive();
                    }
                    Entry::Vacant(v) => {
                        // no existing ladder -> place it
                        if inventory.ladder_count > 0 {
                            inventory.ladder_count -= 1;
                            place_horizontal_ladder(
                                &mut commands,
                                model_assets.ladder.clone(),
                                player_direction,
                                player.grid_pos_x as f32,
                                player.grid_pos_y as f32,
                                player_height as f32,
                                v,
                            );
                            player_history
                                .0
                                .push(PlayerHistoryEvent::PlaceHorizontalLadder(key));

                            sound_channel.play(audio_assets.pop.clone());
                        }
                    }
                }
            }
        }
    }
}
