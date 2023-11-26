use bevy::prelude::*;

use crate::{
    level_manager::LevelManager,
    player::{Player, PlayerHistory, PlayerHistoryEvent, PlayerState},
    states::loading::TextureAssets,
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

#[derive(Debug, Component)]
pub struct Countdown(u8);

pub fn spawn_rune(
    commands: &mut Commands,
    texture_assets: &Res<TextureAssets>,
    x: u8,
    y: u8,
    height: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    stamina: u16,
    timestamp: f32,
    countdown: u8,
) {
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
            transform: Transform::from_xyz(x as f32, height, y as f32),
            ..Default::default()
        })
        .insert(RewindRune {
            x,
            y,
            countdown,
            stamina,
            timestamp,
        })
        .with_children(|parent| {
            let mut spawn_countdown = |texture: Handle<Image>, n: u8| {
                parent
                    .spawn(MaterialMeshBundle {
                        mesh: meshes.add(Mesh::from(shape::Plane {
                            size: 0.3,
                            subdivisions: 0,
                        })),
                        material: materials.add(StandardMaterial {
                            base_color_texture: Some(texture),
                            ..Default::default()
                        }),
                        transform: Transform::from_xyz(0.0, 0.3, 0.0)
                            .with_rotation(Quat::from_rotation_x(1.571)),
                        visibility: Visibility::Hidden,
                        ..Default::default()
                    })
                    .insert(Countdown(n));
            };
            spawn_countdown(texture_assets.countdown_4.clone(), 4);
            spawn_countdown(texture_assets.countdown_3.clone(), 3);
            spawn_countdown(texture_assets.countdown_2.clone(), 2);
            spawn_countdown(texture_assets.countdown_1.clone(), 1);
        });
}

pub fn update_countdown_image(
    runes: Query<(&Children, &RewindRune), Changed<RewindRune>>,
    mut countdowns: Query<(&mut Visibility, &Countdown)>,
) {
    for (children, rune) in runes.iter() {
        for child in children.iter() {
            if let Ok((mut visibility, countdown)) = countdowns.get_mut(*child) {
                if rune.countdown == countdown.0 {
                    *visibility = Visibility::Visible;
                } else {
                    *visibility = Visibility::Hidden;
                }
            } else {
                warn!("Rune entity has a child without a countdown component");
            }
        }
    }
}

pub fn handle_rewind_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&Player>,
    texture_assets: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

            spawn_rune(
                &mut commands,
                &texture_assets,
                player.grid_pos_x,
                player.grid_pos_y,
                player_height as f32 + 0.01,
                &mut meshes,
                &mut materials,
                player.stamina,
                time.elapsed_seconds(),
                5,
            );
            inventory.rewind_count -= 1;
            player_history.0.push(PlayerHistoryEvent::PlaceRune);
        }
    }
}
