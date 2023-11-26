use std::collections::hash_map::Entry;

use bevy::prelude::*;

use crate::{
    cave::{spawn_gem, HasGem},
    equipment::{
        ladder::{place_horizontal_ladder, place_vertical_ladder},
        rewind::RewindRune,
        Inventory,
    },
    level_manager::LevelManager,
    player::{Player, PlayerHistoryEvent},
    scale::{spawn_scale, ScaleCounter},
    states::loading::{ModelAssets, TextureAssets},
    util::{Alignment, CardinalDirection},
};

pub fn handle_undo_player_move(
    mut undo_event_reader: EventReader<PlayerHistoryEvent>,
    mut player: Query<&mut Player>,
    mut rewind_runes: Query<(Entity, &mut RewindRune)>,
) {
    for event in undo_event_reader.iter() {
        match event {
            PlayerHistoryEvent::PlayerMove(old_player)
            | PlayerHistoryEvent::PlayerMoveToScale(old_player)
            | PlayerHistoryEvent::PlayerMoveToGem(old_player) => {
                let mut player = player
                    .get_single_mut()
                    .expect("There should only be one player");
                *player = old_player.clone();

                // undo rune countdowns
                for (_, mut rune) in rewind_runes.iter_mut() {
                    rune.countdown += 1;
                }
            }
            _ => {}
        }
    }
}

pub fn handle_undo_collect_scale(
    mut commands: Commands,
    mut undo_event_reader: EventReader<PlayerHistoryEvent>,
    level_manager: Res<LevelManager>,
    mut scale_counter: ResMut<ScaleCounter>,
    model_assets: Res<ModelAssets>,
) {
    for event in undo_event_reader.iter() {
        if let PlayerHistoryEvent::PlayerMoveToScale(_) = event {
            // Spawn scale
            let map = &level_manager.get_current_level().map;
            if let Some((scale_x, scale_y)) = map.scale_pos {
                spawn_scale(
                    &mut commands,
                    scale_x,
                    scale_y,
                    map.grid_heights[scale_y as usize][scale_x as usize],
                    model_assets.scale.clone(),
                );
            }
            // decrement scale counter
            if scale_counter.0 > 0 {
                scale_counter.0 -= 1;
            } else {
                warn!("Un-did scale pickup, but the scale counter was zero!");
            }
        }
    }
}

pub fn handle_undo_collect_gem(
    mut commands: Commands,
    mut undo_event_reader: EventReader<PlayerHistoryEvent>,
    level_manager: Res<LevelManager>,
    mut has_gem: ResMut<HasGem>,
    model_assets: Res<ModelAssets>,
) {
    for event in undo_event_reader.iter() {
        if let PlayerHistoryEvent::PlayerMoveToGem(_) = event {
            // Spawn gem
            let map = &level_manager.get_current_level().map;
            if let Some(cave_data) = &map.cave_data {
                if let Some((gem_x, gem_y)) = cave_data.gem_pos {
                    spawn_gem(
                        &mut commands,
                        gem_x,
                        gem_y,
                        map.grid_heights[gem_y as usize][gem_x as usize] as f32 + 0.3,
                        model_assets.gem.clone(),
                    );
                }
            }
            // no longer have gem
            has_gem.0 = false;
        }
    }
}

pub fn handle_undo_place_item(
    mut commands: Commands,
    mut undo_event_reader: EventReader<PlayerHistoryEvent>,
    mut level_manager: ResMut<LevelManager>,
    mut inventory: ResMut<Inventory>,
    rewind_runes: Query<(Entity, &RewindRune)>,
) {
    for event in undo_event_reader.iter() {
        match event {
            PlayerHistoryEvent::PlaceVerticalLadder(key) => {
                if let Some(entity) = level_manager
                    .get_current_map_mut()
                    .vertical_ladders
                    .remove(key)
                {
                    commands.entity(entity).despawn_recursive();
                    inventory.ladder_count += 1;
                } else {
                    warn!("Tried to undo vertical ladder placement, but it didn't exist!");
                }
            }
            PlayerHistoryEvent::PlaceHorizontalLadder(key) => {
                if let Some(entity) = level_manager
                    .get_current_map_mut()
                    .horizontal_ladders
                    .remove(key)
                {
                    commands.entity(entity).despawn_recursive();
                    inventory.ladder_count += 1;
                } else {
                    warn!("Tried to undo horizontal ladder placement, but it didn't exist!");
                }
            }
            PlayerHistoryEvent::PlaceRope(key) => {
                if let Some(entity) = level_manager.get_current_map_mut().ropes.remove(key) {
                    commands.entity(entity).despawn_recursive();
                    inventory.rope_count += 1;
                } else {
                    warn!("Tried to undo rope placement, but it didn't exist!");
                }
            }
            PlayerHistoryEvent::PlaceRune => {
                // find the most recently placed rune and delete it
                let mut most_recent = None;
                let mut most_recent_timestamp = 0.0;
                for (entity, rune) in rewind_runes.iter() {
                    if rune.timestamp > most_recent_timestamp {
                        most_recent = Some(entity);
                        most_recent_timestamp = rune.timestamp;
                    }
                }
                if let Some(entity) = most_recent {
                    commands.entity(entity).despawn_recursive();
                    inventory.rewind_count += 1;
                } else {
                    warn!("Tried to undo rune placement, but no runes exist!");
                }
            }
            _ => (),
        }
    }
}

pub fn handle_undo_pickup_ladder(
    mut commands: Commands,
    mut undo_event_reader: EventReader<PlayerHistoryEvent>,
    player: Query<&Player>,
    mut inventory: ResMut<Inventory>,
    mut level_manager: ResMut<LevelManager>,
    model_assets: Res<ModelAssets>,
) {
    for event in undo_event_reader.iter() {
        match event {
            PlayerHistoryEvent::PickUpVerticalLadder(key) => {
                match level_manager
                    .get_current_map_mut()
                    .vertical_ladders
                    .entry(key.clone())
                {
                    Entry::Occupied(_) => {
                        warn!("Tried to undo ladder pickup, but a ladder was already there!")
                    }
                    Entry::Vacant(v) => {
                        let key = v.key();
                        let player = player
                            .get_single()
                            .expect("There should only be one player");
                        place_vertical_ladder(
                            &mut commands,
                            model_assets.ladder.clone(),
                            key.direction,
                            player.grid_pos_x as f32,
                            player.grid_pos_y as f32,
                            key.height as f32,
                            v,
                        );
                        inventory.ladder_count -= 1;
                    }
                }
            }
            PlayerHistoryEvent::PickUpHorizontalLadder(key) => {
                match level_manager
                    .get_current_map_mut()
                    .horizontal_ladders
                    .entry(key.clone())
                {
                    Entry::Occupied(_) => {
                        warn!("Tried to undo ladder pickup, but a ladder was already there!")
                    }
                    Entry::Vacant(v) => {
                        let key = v.key();
                        let player = player
                            .get_single()
                            .expect("There should only be one player");
                        let direction = match key.alignment {
                            Alignment::Xaxis => CardinalDirection::East,
                            Alignment::Yaxis => CardinalDirection::North,
                        };
                        place_horizontal_ladder(
                            &mut commands,
                            model_assets.ladder.clone(),
                            direction,
                            player.grid_pos_x as f32,
                            player.grid_pos_y as f32,
                            key.height as f32,
                            v,
                        );
                        inventory.ladder_count -= 1;
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn handle_undo_teleport(
    mut commands: Commands,
    mut undo_event_reader: EventReader<PlayerHistoryEvent>,
    player: Query<&Player>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    level_manager: Res<LevelManager>,
    texture_assets: Res<TextureAssets>,
) {
    for event in undo_event_reader.iter() {
        if let PlayerHistoryEvent::Teleport((x, y, timestamp)) = event {
            let player = player
                .get_single()
                .expect("There should only be one player");
            let player_height =
                level_manager.get_current_level().map.grid_heights[*y as usize][*x as usize];
            commands
                .spawn(MaterialMeshBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane {
                        size: 1.0,
                        subdivisions: 0,
                    })),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(texture_assets.rune_circle.clone()),
                        alpha_mode: AlphaMode::Blend,
                        ..Default::default()
                    }),
                    transform: Transform::from_xyz(
                        player.grid_pos_x as f32,
                        player_height as f32 + 0.01,
                        player.grid_pos_y as f32,
                    ),
                    ..Default::default()
                })
                .insert(RewindRune {
                    x: *x,
                    y: *y,
                    countdown: 1,
                    stamina: player.stamina,
                    timestamp: *timestamp,
                });
        }
    }
}
