use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    audio::{AudioAssets, SoundChannel},
    level_manager::LevelManager,
    player::{Player, PlayerHistory, PlayerHistoryEvent},
    states::level::DespawnOnTransition,
    util::Spin,
};

#[derive(Default, Resource)]
pub struct HasGem(pub bool);

#[derive(Debug, Reflect)]
pub struct CaveData {
    pub first_pos: (u8, u8),
    pub second_pos: (u8, u8),
    pub gem_pos: Option<(u8, u8)>,
}

#[derive(Component)]
pub struct Cave;

#[derive(Component)]
pub struct GemCave;

#[derive(Component)]
pub struct Gem;

pub fn spawn_gem(commands: &mut Commands, x: u8, y: u8, height: f32, scene: Handle<Scene>) {
    commands
        .spawn(SceneBundle {
            scene,
            transform: Transform::from_xyz(x as f32, height, y as f32),
            ..Default::default()
        })
        .insert(Spin(height))
        .insert(Gem)
        .insert(Name::new("Gem"))
        .insert(DespawnOnTransition);
}

pub fn swap_cave_visibility(
    mut caves: Query<&mut Visibility, (With<Cave>, Without<GemCave>)>,
    mut gem_caves: Query<&mut Visibility, (With<GemCave>, Without<Cave>)>,
    has_gem: Res<HasGem>,
) {
    if has_gem.0 {
        for mut visibility in caves.iter_mut() {
            *visibility = Visibility::Visible;
        }
        for mut visibility in gem_caves.iter_mut() {
            *visibility = Visibility::Hidden;
        }
    } else {
        for mut visibility in caves.iter_mut() {
            *visibility = Visibility::Hidden;
        }
        for mut visibility in gem_caves.iter_mut() {
            *visibility = Visibility::Visible;
        }
    }
}

pub fn check_if_at_gem(
    mut commands: Commands,
    player: Query<&Player>,
    mut player_history: ResMut<PlayerHistory>,
    level_manager: Res<LevelManager>,
    gems: Query<Entity, With<Gem>>,
    mut has_gem: ResMut<HasGem>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    let map = &level_manager.get_current_level().map;
    if let Some(cave_data) = &map.cave_data {
        if let Some((x, y)) = cave_data.gem_pos {
            let player = player
                .get_single()
                .expect("There should only be one player");

            if player.grid_pos_x == x && player.grid_pos_y == y {
                for entity in gems.iter() {
                    // should only be a single iteration
                    commands.entity(entity).despawn_recursive();
                    sound_channel.play(audio_assets.pickup.clone());
                    if let Some(PlayerHistoryEvent::PlayerMove(old_player)) = player_history.0.pop()
                    {
                        player_history
                            .0
                            .push(PlayerHistoryEvent::PlayerMoveToGem(old_player));
                    } else {
                        warn!("Failed to add scale pickup event to history");
                    }
                    has_gem.0 = true;
                }
            }
        }
    }
}
