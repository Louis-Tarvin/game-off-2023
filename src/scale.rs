use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    audio::{AudioAssets, SoundChannel},
    level_manager::LevelManager,
    player::{Player, PlayerHistory, PlayerHistoryEvent, PlayerState},
    states::level::DespawnOnTransition,
    util::Spin,
};

#[derive(Debug, Default, Resource, Reflect)]
pub struct ScaleCounter(pub u8);

#[derive(Component)]
pub struct Scale;

pub fn spawn_scale(commands: &mut Commands, x: u8, y: u8, height: u8, scene: Handle<Scene>) {
    let mut transform = Transform::from_xyz(x as f32, height as f32 + 0.3, y as f32);
    transform.rotate_local_x(-0.2);
    commands
        .spawn(SceneBundle {
            scene,
            transform,
            ..Default::default()
        })
        .insert(Spin(height as f32 + 0.3))
        .insert(Scale)
        .insert(Name::new("Scale"))
        .insert(DespawnOnTransition);
}

pub fn rotation(mut query: Query<(&mut Transform, &Spin)>, time: Res<Time>) {
    let rotation_speed = 1.0;
    for (mut transform, spin) in query.iter_mut() {
        // spin
        transform.rotate_y(rotation_speed * time.delta_seconds());
        // bob up and down
        transform.translation.y = spin.0 + 0.1 * time.elapsed_seconds().sin();
    }
}

pub fn check_if_at_scale(
    mut commands: Commands,
    player: Query<&Player>,
    mut player_history: ResMut<PlayerHistory>,
    level_manager: Res<LevelManager>,
    scale_entities: Query<Entity, With<Scale>>,
    mut scale_counter: ResMut<ScaleCounter>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    let map = &level_manager.get_current_level().map;
    if let Some((x, y)) = map.scale_pos {
        let player = player
            .get_single()
            .expect("There should only be one player");

        let mut at_scale = player.grid_pos_x == x && player.grid_pos_y == y;
        if let PlayerState::Climbing(climb_state) = &player.state {
            at_scale &= map.grid_heights[y as usize][x as usize] == climb_state.elevation - 1;
        }

        if at_scale {
            for entity in scale_entities.iter() {
                // should only be a single iteration
                commands.entity(entity).despawn_recursive();
                sound_channel.play(audio_assets.pickup.clone());
                if let Some(PlayerHistoryEvent::PlayerMove(old_player)) = player_history.0.pop() {
                    player_history
                        .0
                        .push(PlayerHistoryEvent::PlayerMoveToScale(old_player));
                } else {
                    warn!("Failed to add scale pickup event to history");
                }
                scale_counter.0 += 1;
            }
        }
    }
}
