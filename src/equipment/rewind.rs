use bevy::prelude::*;

use crate::{
    level_manager::LevelManager,
    player::{Player, PlayerHistory, PlayerHistoryEvent, PlayerState},
    states::loading::ModelAssets,
};

use super::Inventory;

#[derive(Debug, Component)]
pub struct RewindRune {
    pub x: u8,
    pub y: u8,
    pub countdown: u8,
    pub stamina: u16,
    pub timestamp: f32,
}

pub fn handle_rewind_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&Player>,
    model_assets: Res<ModelAssets>,
    level_manager: Res<LevelManager>,
    mut player_history: ResMut<PlayerHistory>,
    mut inventory: ResMut<Inventory>,
    time: Res<Time>,
) {
    if let Ok(player) = player_query.get_single() {
        if keyboard_input.just_pressed(KeyCode::Key3)
            && matches!(player.state, PlayerState::Standing(_))
            && inventory.rewind_count > 0
        {
            let player_height = level_manager.get_current_level().map.grid_heights
                [player.grid_pos_y as usize][player.grid_pos_x as usize];

            commands
                .spawn(SceneBundle {
                    scene: model_assets.rune.clone(),
                    transform: Transform::from_xyz(
                        player.grid_pos_x as f32,
                        player_height as f32 + 0.01,
                        player.grid_pos_y as f32,
                    ),
                    ..Default::default()
                })
                .insert(RewindRune {
                    x: player.grid_pos_x,
                    y: player.grid_pos_y,
                    countdown: 5,
                    stamina: player.stamina,
                    timestamp: time.elapsed_seconds(),
                });
            inventory.rewind_count -= 1;
            player_history.0.push(PlayerHistoryEvent::PlaceRune);
        }
    }
}
