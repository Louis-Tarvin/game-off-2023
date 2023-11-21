use std::collections::hash_map::Entry;

use bevy::prelude::*;

use crate::{
    equipment::{
        ladder::{
            place_horizontal_ladder, place_vertical_ladder, HorizontalLadderKey, VerticalLadderKey,
        },
        rewind::RewindRune,
        rope::RopeKey,
        Inventory,
    },
    level_manager::LevelManager,
    map::Map,
    scale::check_if_at_scale,
    states::{
        level::DespawnOnTransition, loading::ModelAssets, transition::TransitionManager, GameState,
    },
    ui::equipment::{InfoUiRoot, PickingUiRoot},
    util::{Alignment, CardinalDirection},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .register_type::<PlayerHistory>()
            .insert_resource(PlayerHistory::default())
            .add_systems(OnEnter(GameState::Level), spawn_player)
            .add_systems(
                Update,
                (
                    player_input,
                    update_player_position,
                    check_if_at_flag,
                    check_if_at_scale,
                )
                    .chain()
                    .run_if(in_state(GameState::Level)),
            );
    }
}

const MOVE_STAMINA: u16 = 1;
const CLIMB_UP_STAMINA: u16 = 4;
const CLIMB_SIDEWAYS_STAMINA: u16 = 2;
const CLIMB_DOWN_STAMINA: u16 = 2;

#[derive(Debug, Clone, Reflect)]
pub enum PlayerState {
    Standing(CardinalDirection),
    Climbing(ClimbingState),
    StandingOnLadder(LadderState),
}

#[derive(Debug, Clone, Reflect)]
pub struct ClimbingState {
    pub direction: CardinalDirection,
    pub elevation: u8,
}

#[derive(Debug, Clone, Reflect)]
pub struct LadderState {
    pub direction: CardinalDirection,
    pub elevation: u8,
    pub alignment: Alignment,
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct Player {
    pub stamina: u16,
    pub grid_pos_x: u8,
    pub grid_pos_y: u8,
    pub state: PlayerState,
}
impl Player {
    pub fn go(&self, direction: CardinalDirection, map: &Map) -> Option<Self> {
        let heights = &map.grid_heights;
        let x = self.grid_pos_x as usize;
        let y = self.grid_pos_y as usize;

        // check if changing direction
        if let PlayerState::Standing(facing) = self.state {
            if facing != direction {
                return Some(Self {
                    stamina: self.stamina,
                    grid_pos_x: self.grid_pos_x,
                    grid_pos_y: self.grid_pos_y,
                    state: PlayerState::Standing(direction),
                });
            }
        }

        // check if moving out of bounds
        if matches!(self.state, PlayerState::Standing(_))
            || matches!(self.state, PlayerState::StandingOnLadder(_))
        {
            match direction {
                CardinalDirection::North => {
                    if y == 0 {
                        return None;
                    }
                }
                CardinalDirection::East => {
                    if x == heights[0].len() - 1 {
                        return None;
                    }
                }
                CardinalDirection::South => {
                    if y == heights.len() - 1 {
                        return None;
                    }
                }
                CardinalDirection::West => {
                    if x == 0 {
                        return None;
                    }
                }
            }
        }

        let current_elevation = heights[y][x];
        match &self.state {
            PlayerState::Standing(_) => {
                let (new_x, new_y) = match direction {
                    CardinalDirection::North => (x, y - 1),
                    CardinalDirection::East => (x + 1, y),
                    CardinalDirection::South => (x, y + 1),
                    CardinalDirection::West => (x - 1, y),
                };
                // equal elevation
                if heights[new_y][new_x] == current_elevation {
                    self.stamina.checked_sub(MOVE_STAMINA).map(|stamina| Self {
                        stamina,
                        grid_pos_x: new_x as u8,
                        grid_pos_y: new_y as u8,
                        state: PlayerState::Standing(direction),
                    })
                } else if heights[new_y][new_x] > current_elevation {
                    // elevation rises -> check if wall is climbable or is a ladder/rope
                    if map.grid_climbable[new_y][new_x]
                        || map.is_ladder_or_rope(
                            self.grid_pos_x,
                            self.grid_pos_y,
                            current_elevation + 1,
                            direction,
                        )
                    {
                        // cling to wall
                        Some(Self {
                            stamina: self.stamina,
                            grid_pos_x: self.grid_pos_x,
                            grid_pos_y: self.grid_pos_y,
                            state: PlayerState::Climbing(ClimbingState {
                                direction,
                                elevation: current_elevation + 1,
                            }),
                        })
                    } else {
                        None
                    }
                } else if map.horizontal_ladders.contains_key(&HorizontalLadderKey {
                    x: new_x as u8,
                    y: new_y as u8,
                    height: current_elevation,
                    alignment: direction.into(),
                }) {
                    // elevation drops -> check if there is a horizontal ladder
                    self.stamina.checked_sub(MOVE_STAMINA).map(|stamina| Self {
                        stamina,
                        grid_pos_x: new_x as u8,
                        grid_pos_y: new_y as u8,
                        state: PlayerState::StandingOnLadder(LadderState {
                            direction,
                            elevation: current_elevation,
                            alignment: direction.into(),
                        }),
                    })
                } else if map.grid_climbable[self.grid_pos_y as usize][self.grid_pos_x as usize]
                    || map.is_ladder_or_rope(
                        new_x as u8,
                        new_y as u8,
                        current_elevation,
                        direction.reverse(),
                    )
                {
                    // climbing down
                    let cost = if map.is_ladder_or_rope(
                        new_x as u8,
                        new_y as u8,
                        current_elevation,
                        direction.reverse(),
                    ) {
                        1
                    } else {
                        CLIMB_DOWN_STAMINA
                    };
                    self.stamina.checked_sub(cost).map(|stamina| Self {
                        stamina,
                        grid_pos_x: new_x as u8,
                        grid_pos_y: new_y as u8,
                        state: PlayerState::Climbing(ClimbingState {
                            direction: direction.reverse(),
                            elevation: current_elevation,
                        }),
                    })
                } else {
                    None
                }
            }
            PlayerState::Climbing(climb_state) => match direction {
                CardinalDirection::North => {
                    let (x_offset, y_offset) = match &climb_state.direction {
                        CardinalDirection::North => (0, -1),
                        CardinalDirection::East => (1, 0),
                        CardinalDirection::South => (0, 1),
                        CardinalDirection::West => (-1, 0),
                    };
                    // can't climb on boundary so conversion to unsigned is safe
                    let next_x = (x as i16 + x_offset) as usize;
                    let next_y = (y as i16 + y_offset) as usize;
                    let cost = if map.is_ladder_or_rope(
                        self.grid_pos_x,
                        self.grid_pos_y,
                        climb_state.elevation,
                        climb_state.direction,
                    ) {
                        1
                    } else {
                        CLIMB_UP_STAMINA
                    };
                    if cost == 1 || map.grid_climbable[next_y][next_x] {
                        if heights[next_y][next_x] == climb_state.elevation {
                            // climb on top
                            self.stamina.checked_sub(cost).map(|stamina| Self {
                                stamina,
                                grid_pos_x: (next_x) as u8,
                                grid_pos_y: (next_y) as u8,
                                state: PlayerState::Standing(climb_state.direction),
                            })
                        } else {
                            // climb up
                            self.stamina.checked_sub(cost).map(|stamina| Self {
                                stamina,
                                grid_pos_x: self.grid_pos_x,
                                grid_pos_y: self.grid_pos_y,
                                state: PlayerState::Climbing(ClimbingState {
                                    direction: climb_state.direction,
                                    elevation: climb_state.elevation + 1,
                                }),
                            })
                        }
                    } else {
                        None
                    }
                }
                CardinalDirection::East => {
                    let (x_offset, y_offset) = match &climb_state.direction {
                        CardinalDirection::North => (1, 0),
                        CardinalDirection::East => (0, 1),
                        CardinalDirection::South => (-1, 0),
                        CardinalDirection::West => (0, -1),
                    };
                    // grid square moving to
                    let next_x = x as i16 + x_offset;
                    let next_y = y as i16 + y_offset;
                    if next_x < 0
                        || next_x as usize >= heights[0].len()
                        || next_y < 0
                        || next_y as usize >= heights.len()
                    {
                        // out of bounds
                        return None;
                    }
                    let (x_offset, y_offset) = match &climb_state.direction {
                        CardinalDirection::North => (1, -1),
                        CardinalDirection::East => (1, 1),
                        CardinalDirection::South => (-1, 1),
                        CardinalDirection::West => (-1, -1),
                    };
                    // grid square that will be clung to
                    let next_wall_x = x as i16 + x_offset;
                    let next_wall_y = y as i16 + y_offset;
                    if next_wall_x < 0
                        || next_wall_x as usize >= heights[0].len()
                        || next_wall_y < 0
                        || next_wall_y as usize >= heights.len()
                    {
                        // out of bounds
                        return None;
                    }
                    if !map.grid_climbable[next_wall_y as usize][next_wall_x as usize] {
                        // not climbable
                        return None;
                    }
                    if heights[next_y as usize][next_x as usize] >= climb_state.elevation
                        || heights[next_wall_y as usize][next_wall_x as usize]
                            < climb_state.elevation
                    {
                        // no valid climb spot to the east
                        None
                    } else {
                        // move east
                        self.stamina
                            .checked_sub(CLIMB_SIDEWAYS_STAMINA)
                            .map(|stamina| Self {
                                stamina,
                                grid_pos_x: next_x as u8,
                                grid_pos_y: next_y as u8,
                                state: PlayerState::Climbing(ClimbingState {
                                    direction: climb_state.direction,
                                    elevation: climb_state.elevation,
                                }),
                            })
                    }
                }
                CardinalDirection::South => {
                    if current_elevation + 1 == climb_state.elevation {
                        // dismount wall
                        Some(Self {
                            stamina: self.stamina,
                            grid_pos_x: self.grid_pos_x,
                            grid_pos_y: self.grid_pos_y,
                            state: PlayerState::Standing(climb_state.direction),
                        })
                    } else {
                        // climb down
                        let cost = if map.is_ladder_or_rope(
                            self.grid_pos_x,
                            self.grid_pos_y,
                            climb_state.elevation,
                            climb_state.direction,
                        ) {
                            1
                        } else {
                            CLIMB_DOWN_STAMINA
                        };
                        if let Some(stamina) = self.stamina.checked_sub(cost) {
                            climb_state.elevation.checked_sub(1).map(|elevation| Self {
                                stamina,
                                grid_pos_x: self.grid_pos_x,
                                grid_pos_y: self.grid_pos_y,
                                state: PlayerState::Climbing(ClimbingState {
                                    direction: climb_state.direction,
                                    elevation,
                                }),
                            })
                        } else {
                            None
                        }
                    }
                }
                CardinalDirection::West => {
                    let (x_offset, y_offset) = match &climb_state.direction {
                        CardinalDirection::North => (-1, 0),
                        CardinalDirection::East => (0, -1),
                        CardinalDirection::South => (1, 0),
                        CardinalDirection::West => (0, 1),
                    };
                    // grid square moving to
                    let next_x = x as i16 + x_offset;
                    let next_y = y as i16 + y_offset;
                    if next_x < 0
                        || next_x as usize >= heights[0].len()
                        || next_y < 0
                        || next_y as usize >= heights.len()
                    {
                        // out of bounds
                        return None;
                    }
                    let (x_offset, y_offset) = match &climb_state.direction {
                        CardinalDirection::North => (-1, -1),
                        CardinalDirection::East => (1, -1),
                        CardinalDirection::South => (1, 1),
                        CardinalDirection::West => (-1, 1),
                    };
                    // grid square that will be clung to
                    let next_wall_x = x as i16 + x_offset;
                    let next_wall_y = y as i16 + y_offset;
                    if next_wall_x < 0
                        || next_wall_x as usize >= heights[0].len()
                        || next_wall_y < 0
                        || next_wall_y as usize >= heights.len()
                    {
                        // out of bounds
                        return None;
                    }
                    if !map.grid_climbable[next_wall_y as usize][next_wall_x as usize] {
                        // not climbable
                        return None;
                    }
                    if heights[next_y as usize][next_x as usize] >= climb_state.elevation
                        || heights[next_wall_y as usize][next_wall_x as usize]
                            < climb_state.elevation
                    {
                        // no valid climb spot to the west
                        None
                    } else {
                        // move west
                        self.stamina
                            .checked_sub(CLIMB_SIDEWAYS_STAMINA)
                            .map(|stamina| Self {
                                stamina,
                                grid_pos_x: next_x as u8,
                                grid_pos_y: next_y as u8,
                                state: PlayerState::Climbing(ClimbingState {
                                    direction: climb_state.direction,
                                    elevation: climb_state.elevation,
                                }),
                            })
                    }
                }
            },
            PlayerState::StandingOnLadder(ladder_state) => {
                let (new_x, new_y) = match direction {
                    CardinalDirection::North => (x, y - 1),
                    CardinalDirection::East => (x + 1, y),
                    CardinalDirection::South => (x, y + 1),
                    CardinalDirection::West => (x - 1, y),
                };
                #[allow(clippy::comparison_chain)]
                if heights[new_y][new_x] == ladder_state.elevation {
                    {
                        // move off ladder
                        self.stamina.checked_sub(MOVE_STAMINA).map(|stamina| Self {
                            stamina,
                            grid_pos_x: new_x as u8,
                            grid_pos_y: new_y as u8,
                            state: PlayerState::Standing(direction),
                        })
                    }
                } else if heights[new_y][new_x] > ladder_state.elevation {
                    {
                        // climb off ladder
                        Some(Self {
                            stamina: self.stamina,
                            grid_pos_x: new_x as u8,
                            grid_pos_y: new_y as u8,
                            state: PlayerState::Climbing(ClimbingState {
                                direction,
                                elevation: ladder_state.elevation + 1,
                            }),
                        })
                    }
                } else {
                    None
                }
            }
        }
    }

    fn has_direction_changed(&self, other: &Player) -> bool {
        if let PlayerState::Standing(dir1) = self.state {
            if let PlayerState::Standing(dir2) = other.state {
                return dir1 != dir2;
            }
        }
        false
    }
}

fn spawn_player(
    mut commands: Commands,
    model_assets: Res<ModelAssets>,
    level_manager: Res<LevelManager>,
) {
    let map = &level_manager.get_current_level().map;
    commands
        .spawn(SceneBundle {
            scene: model_assets.climber.clone(),
            ..Default::default()
        })
        .insert(Player {
            stamina: level_manager.get_current_level().stamina_budget,
            grid_pos_x: map.player_start_pos.0,
            grid_pos_y: map.player_start_pos.1,
            state: PlayerState::Standing(CardinalDirection::South),
        })
        .insert(Transform::from_xyz(
            map.player_start_pos.0 as f32,
            map.grid_heights[map.player_start_pos.1 as usize][map.player_start_pos.0 as usize]
                as f32,
            map.player_start_pos.1 as f32,
        ))
        .insert(Name::new("Climber"))
        .insert(DespawnOnTransition);
}

fn update_player_position(
    mut query: Query<(&mut Transform, &Player)>,
    level_manager: Res<LevelManager>,
) {
    for (mut transform, player) in query.iter_mut() {
        match &player.state {
            PlayerState::Standing(direction) => {
                transform.translation = Vec3::new(
                    player.grid_pos_x as f32,
                    level_manager.get_current_level().map.grid_heights[player.grid_pos_y as usize]
                        [player.grid_pos_x as usize] as f32,
                    player.grid_pos_y as f32,
                );
                transform.look_to((*direction).into(), Vec3::Y);
            }
            PlayerState::Climbing(climb_state) => {
                transform.look_to(climb_state.direction.into(), Vec3::Y);
                let (x_offset, y_offset) = match climb_state.direction {
                    CardinalDirection::North => (0., -0.4),
                    CardinalDirection::East => (0.4, 0.),
                    CardinalDirection::South => (0., 0.4),
                    CardinalDirection::West => (-0.4, 0.),
                };
                transform.translation = Vec3::new(
                    player.grid_pos_x as f32 + x_offset,
                    climb_state.elevation as f32 - 0.8,
                    player.grid_pos_y as f32 + y_offset,
                )
            }
            PlayerState::StandingOnLadder(ladder_state) => {
                transform.translation = Vec3::new(
                    player.grid_pos_x as f32,
                    ladder_state.elevation as f32,
                    player.grid_pos_y as f32,
                );
                transform.look_to(ladder_state.direction.into(), Vec3::Y);
            }
        }
    }
}

#[derive(Debug, Reflect)]
pub enum PlayerHistoryEvent {
    PlayerMove(Player),
    PlaceVerticalLadder(VerticalLadderKey),
    PlaceHorizontalLadder(HorizontalLadderKey),
    PlaceRope(RopeKey),
    PickUpVerticalLadder(VerticalLadderKey),
    PickUpHorizontalLadder(HorizontalLadderKey),
    PlaceRune,
    // (x, y, timestamp)
    Teleport((u8, u8, f32)),
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct PlayerHistory(pub Vec<PlayerHistoryEvent>);

fn player_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<&mut Player>,
    mut player_history: ResMut<PlayerHistory>,
    mut level_manager: ResMut<LevelManager>,
    mut picking_ui: Query<&mut Visibility, With<PickingUiRoot>>,
    mut info_ui: Query<&mut Visibility, (With<InfoUiRoot>, Without<PickingUiRoot>)>,
    mut inventory: ResMut<Inventory>,
    model_assets: Res<ModelAssets>,
    mut rewind_runes: Query<(Entity, &mut RewindRune)>,
) {
    let mut direction = None;
    if keyboard_input.any_just_pressed([KeyCode::W, KeyCode::Up]) {
        direction = Some(CardinalDirection::North);
    } else if keyboard_input.any_just_pressed([KeyCode::D, KeyCode::Right]) {
        direction = Some(CardinalDirection::East);
    } else if keyboard_input.any_just_pressed([KeyCode::S, KeyCode::Down]) {
        direction = Some(CardinalDirection::South);
    } else if keyboard_input.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        direction = Some(CardinalDirection::West);
    } else if keyboard_input.just_pressed(KeyCode::Z)
        && keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
    {
        // undo the last move
        if let Some(event) = player_history.0.pop() {
            match event {
                PlayerHistoryEvent::PlayerMove(old_player) => {
                    let mut player = player
                        .get_single_mut()
                        .expect("There should only be one player");
                    *player = old_player;

                    // undo rune countdowns
                    for (_, mut rune) in rewind_runes.iter_mut() {
                        rune.countdown += 1;
                    }
                }
                PlayerHistoryEvent::PlaceVerticalLadder(key) => {
                    if let Some(entity) = level_manager
                        .get_current_map_mut()
                        .vertical_ladders
                        .remove(&key)
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
                        .remove(&key)
                    {
                        commands.entity(entity).despawn_recursive();
                        inventory.ladder_count += 1;
                    } else {
                        warn!("Tried to undo horizontal ladder placement, but it didn't exist!");
                    }
                }
                PlayerHistoryEvent::PlaceRope(key) => {
                    if let Some(entity) = level_manager.get_current_map_mut().ropes.remove(&key) {
                        commands.entity(entity).despawn_recursive();
                        inventory.rope_count += 1;
                    } else {
                        warn!("Tried to undo rope placement, but it didn't exist!");
                    }
                }
                PlayerHistoryEvent::PickUpVerticalLadder(key) => {
                    match level_manager
                        .get_current_map_mut()
                        .vertical_ladders
                        .entry(key)
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
                                commands,
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
                        .entry(key)
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
                                commands,
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
                PlayerHistoryEvent::Teleport((x, y, timestamp)) => {
                    let player = player
                        .get_single()
                        .expect("There should only be one player");
                    let player_height =
                        level_manager.get_current_level().map.grid_heights[y as usize][x as usize];
                    commands
                        .spawn(SceneBundle {
                            scene: model_assets.rune.clone(),
                            transform: Transform::from_xyz(
                                x as f32,
                                player_height as f32 + 0.01,
                                y as f32,
                            ),
                            ..Default::default()
                        })
                        .insert(RewindRune {
                            x,
                            y,
                            countdown: 1,
                            stamina: player.stamina,
                            timestamp,
                        });
                }
            }
        } else {
            // swap UI
            *picking_ui.get_single_mut().unwrap() = Visibility::Visible;
            *info_ui.get_single_mut().unwrap() = Visibility::Hidden;
        }
        return;
    }

    if let Some(direction) = direction {
        let mut player = player
            .get_single_mut()
            .expect("There should only be one player");
        let new_player = player.go(direction, &level_manager.get_current_level().map);
        if let Some(new_player) = new_player {
            let mut teleported = false;
            if !player.has_direction_changed(&new_player) {
                // Player has moved
                player_history
                    .0
                    .push(PlayerHistoryEvent::PlayerMove(player.clone()));
                // decrement the counters of each rewind rune. If it has reached 0 we teleport
                for (entity, mut rune) in rewind_runes.iter_mut() {
                    rune.countdown -= 1;
                    if rune.countdown == 0 {
                        // teleport the player
                        player.grid_pos_x = rune.x;
                        player.grid_pos_y = rune.y;
                        player.stamina = rune.stamina;
                        player.state = PlayerState::Standing(CardinalDirection::South);
                        teleported = true;
                        commands.entity(entity).despawn_recursive();
                        player_history.0.push(PlayerHistoryEvent::Teleport((
                            rune.x,
                            rune.y,
                            rune.timestamp,
                        )))
                    }
                }
            }
            if !teleported {
                *player = new_player;
                // swap UI
                *picking_ui.get_single_mut().unwrap() = Visibility::Hidden;
                *info_ui.get_single_mut().unwrap() = Visibility::Visible;
            }
        }
    }
}

fn check_if_at_flag(
    player: Query<&Player>,
    level_manager: Res<LevelManager>,
    mut transition_manager: ResMut<TransitionManager>,
) {
    let player = player
        .get_single()
        .expect("There should only be one player");

    let map = &level_manager.get_current_level().map;
    if matches!(*transition_manager, TransitionManager::Normal)
        && player.grid_pos_x == map.flag_pos.0
        && player.grid_pos_y == map.flag_pos.1
    {
        *transition_manager = TransitionManager::TransitioningOut(0.0);
    }
}

pub fn clear_player_history(mut player_history: ResMut<PlayerHistory>) {
    player_history.0.clear();
}
