use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    audio::{AudioAssets, SoundChannel},
    level_manager::LevelManager,
    player::Player,
};

#[derive(Component)]
pub struct Scale(pub f32);

pub fn scale_rotation(mut scales: Query<(&mut Transform, &Scale)>, time: Res<Time>) {
    let rotation_speed = 1.0;
    for (mut scale_transform, scale) in scales.iter_mut() {
        // spin
        scale_transform.rotate_y(rotation_speed * time.delta_seconds());
        // bob up and down
        scale_transform.translation.y = scale.0 + 0.1 * time.elapsed_seconds().sin();
    }
}

pub fn check_if_at_scale(
    mut commands: Commands,
    player: Query<&Player>,
    level_manager: Res<LevelManager>,
    scale_entities: Query<Entity, With<Scale>>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    let map = &level_manager.get_current_level().map;
    if let Some((x, y)) = map.scale_pos {
        let player = player
            .get_single()
            .expect("There should only be one player");

        if player.grid_pos_x == x && player.grid_pos_y == y {
            for entity in scale_entities.iter() {
                commands.entity(entity).despawn_recursive();
                sound_channel.play(audio_assets.pickup.clone());
            }
            // TODO: add point
        }
    }
}
