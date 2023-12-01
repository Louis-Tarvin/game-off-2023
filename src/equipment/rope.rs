use std::collections::hash_map::Entry;

use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    audio::{AudioAssets, SoundChannel},
    level_manager::LevelManager,
    player::{Player, PlayerHistory, PlayerHistoryEvent, PlayerState},
    states::{level::DespawnOnTransition, loading::ModelAssets},
    util::CardinalDirection,
};

use super::Inventory;

#[derive(Debug, PartialEq, Eq, Hash, Reflect, Clone)]
pub struct RopeKey {
    pub x: u8,
    pub y: u8,
    pub direction: CardinalDirection,
}

pub fn handle_rope_input(
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
    if keyboard_input.just_pressed(KeyCode::Key2) {
        let map = level_manager.get_current_map_mut();
        let player = player_query
            .get_single()
            .expect("There should only be one player");
        if let PlayerState::Standing(direction) = player.state {
            let player_height =
                map.grid_heights[player.grid_pos_y as usize][player.grid_pos_x as usize];
            let (x_offset, y_offset) = match direction {
                CardinalDirection::North => (0, -1),
                CardinalDirection::East => (1, 0),
                CardinalDirection::South => (0, 1),
                CardinalDirection::West => (-1, 0),
            };
            // grid square directly in front of the player (might be out of bounds)
            let grid_facing_x = player.grid_pos_x as i16 + x_offset;
            let grid_facing_y = player.grid_pos_y as i16 + y_offset;

            // Check if there is a valid placement
            if grid_facing_x < 0
                || grid_facing_x >= map.grid_heights[0].len() as i16
                || grid_facing_y < 0
                || grid_facing_y >= map.grid_heights.len() as i16
            {
                sound_channel.play(audio_assets.error.clone());
                return; // out of bounds
            }
            let height_diff = player_height as i16
                - map.grid_heights[grid_facing_y as usize][grid_facing_x as usize] as i16;
            if height_diff > 0 {
                let key = RopeKey {
                    x: player.grid_pos_x,
                    y: player.grid_pos_y,
                    direction,
                };
                match map.ropes.entry(key.clone()) {
                    Entry::Occupied(_) => {}
                    Entry::Vacant(v) => {
                        // Place rope if in inventory
                        if inventory.rope_count > 0 {
                            inventory.rope_count -= 1;
                            let entity = commands
                                .spawn(SceneBundle {
                                    scene: model_assets.rope_top.clone(),
                                    transform: Transform::from_xyz(
                                        player.grid_pos_x as f32,
                                        player_height as f32 - 0.5,
                                        player.grid_pos_y as f32,
                                    )
                                    .looking_to(direction.reverse().into(), Vec3::Y),
                                    ..Default::default()
                                })
                                .insert(Name::new("Rope"))
                                .insert(DespawnOnTransition)
                                .with_children(|parent| {
                                    for i in 0..height_diff {
                                        parent.spawn(SceneBundle {
                                            scene: model_assets.rope.clone(),
                                            transform: Transform::from_xyz(0., -i as f32, 0.),
                                            ..Default::default()
                                        });
                                    }
                                })
                                .id();
                            v.insert(entity);
                            player_history.0.push(PlayerHistoryEvent::PlaceRope(key));
                            sound_channel.play(audio_assets.pop.clone());
                        } else {
                            sound_channel.play(audio_assets.error.clone());
                        }
                    }
                }
            } else {
                sound_channel.play(audio_assets.error.clone());
            }
        }
    }
}
