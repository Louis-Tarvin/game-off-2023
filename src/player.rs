use bevy::prelude::*;

use crate::{
    equipment::HorizontalLadderKey,
    map::Map,
    states::{loading::ModelAssets, GameState},
    util::{Alignment, CardinalDirection},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .insert_resource(PlayerHistory::default())
            .add_systems(OnEnter(GameState::Level), spawn_player)
            .add_systems(
                Update,
                (player_input, update_player_position)
                    .chain()
                    .run_if(in_state(GameState::Level)),
            );
    }
}

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
    pub fn _stamina_cost(&self, direction: CardinalDirection, heights: Vec<Vec<u8>>) -> Option<u8> {
        let x = self.grid_pos_x as usize;
        let y = self.grid_pos_y as usize;
        let current_elevation = heights[y][x];
        match &self.state {
            PlayerState::Standing(_) => {
                let dest_elevation = match direction {
                    CardinalDirection::North => heights[y - 1][x],
                    CardinalDirection::East => heights[y][x + 1],
                    CardinalDirection::South => heights[y + 1][x],
                    CardinalDirection::West => heights[y][x - 1],
                };
                if dest_elevation == current_elevation {
                    Some(1)
                } else {
                    Some(0)
                }
            }
            PlayerState::Climbing(climb_state) => match direction {
                CardinalDirection::North => {
                    if heights[y - 1][x] == climb_state.elevation {
                        Some(2)
                    } else {
                        Some(5)
                    }
                }
                CardinalDirection::East => {
                    if heights[y][x + 1] >= climb_state.elevation
                        || heights[y - 1][x + 1] < climb_state.elevation
                    {
                        None
                    } else {
                        Some(2)
                    }
                }
                CardinalDirection::South => {
                    if current_elevation + 1 == climb_state.elevation {
                        Some(0)
                    } else {
                        Some(1)
                    }
                }
                CardinalDirection::West => {
                    if heights[y][x - 1] >= climb_state.elevation {
                        None
                    } else {
                        Some(2)
                    }
                }
            },
            PlayerState::StandingOnLadder(_) => todo!(),
        }
    }

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
        if let PlayerState::Standing(_) = self.state {
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
                    self.stamina.checked_sub(1).map(|stamina| Self {
                        stamina,
                        grid_pos_x: new_x as u8,
                        grid_pos_y: new_y as u8,
                        state: PlayerState::Standing(direction),
                    })
                } else if heights[new_y][new_x] > current_elevation {
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
                } else if map.horizontal_ladders.contains_key(&HorizontalLadderKey {
                    x: new_x as u8,
                    y: new_y as u8,
                    height: current_elevation,
                    alignment: direction.into(),
                }) {
                    // elevation drops -> check if there is a ladder
                    Some(Self {
                        stamina: self.stamina,
                        grid_pos_x: new_x as u8,
                        grid_pos_y: new_y as u8,
                        state: PlayerState::StandingOnLadder(LadderState {
                            direction,
                            elevation: current_elevation,
                            alignment: direction.into(),
                        }),
                    })
                } else {
                    Some(Self {
                        stamina: self.stamina,
                        grid_pos_x: new_x as u8,
                        grid_pos_y: new_y as u8,
                        state: PlayerState::Climbing(ClimbingState {
                            direction: direction.reverse(),
                            elevation: current_elevation,
                        }),
                    })
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
                    if heights[next_y][next_x] == climb_state.elevation {
                        // climb on top
                        self.stamina.checked_sub(2).map(|stamina| Self {
                            stamina,
                            grid_pos_x: (next_x) as u8,
                            grid_pos_y: (next_y) as u8,
                            state: PlayerState::Standing(climb_state.direction),
                        })
                    } else {
                        // climb up
                        self.stamina.checked_sub(5).map(|stamina| Self {
                            stamina,
                            grid_pos_x: self.grid_pos_x,
                            grid_pos_y: self.grid_pos_y,
                            state: PlayerState::Climbing(ClimbingState {
                                direction: climb_state.direction,
                                elevation: climb_state.elevation + 1,
                            }),
                        })
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
                    if heights[next_y as usize][next_x as usize] >= climb_state.elevation
                        || heights[next_wall_y as usize][next_wall_x as usize]
                            < climb_state.elevation
                    {
                        // no valid climb spot to the east
                        None
                    } else {
                        // move east
                        self.stamina.checked_sub(2).map(|stamina| Self {
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
                        if let Some(stamina) = self.stamina.checked_sub(1) {
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
                    if heights[next_y as usize][next_x as usize] >= climb_state.elevation
                        || heights[next_wall_y as usize][next_wall_x as usize]
                            < climb_state.elevation
                    {
                        // no valid climb spot to the west
                        None
                    } else {
                        // move west
                        self.stamina.checked_sub(2).map(|stamina| Self {
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
                        Some(Self {
                            stamina: self.stamina,
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
}

fn spawn_player(mut commands: Commands, model_assets: Res<ModelAssets>, map: Res<Map>) {
    commands
        .spawn(SceneBundle {
            scene: model_assets.climber.clone(),
            ..Default::default()
        })
        .insert(Player {
            stamina: 100,
            grid_pos_x: map.player_start_pos.0,
            grid_pos_y: map.player_start_pos.1,
            state: PlayerState::Standing(CardinalDirection::South),
        })
        .insert(
            Transform::from_xyz(
                map.player_start_pos.0 as f32,
                map.grid_heights[map.player_start_pos.1 as usize][map.player_start_pos.0 as usize]
                    as f32,
                map.player_start_pos.1 as f32,
            )
            .with_scale(Vec3::splat(0.5)),
        )
        .insert(Name::new("Climber"));
}

fn update_player_position(mut query: Query<(&mut Transform, &Player)>, map: Res<Map>) {
    for (mut transform, player) in query.iter_mut() {
        match &player.state {
            PlayerState::Standing(direction) => {
                transform.translation = Vec3::new(
                    player.grid_pos_x as f32,
                    map.grid_heights[player.grid_pos_y as usize][player.grid_pos_x as usize] as f32,
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

#[derive(Debug, Default, Resource, Reflect)]
pub struct PlayerHistory(Vec<Player>);

fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Player>,
    mut player_history: ResMut<PlayerHistory>,
    map: Res<Map>,
) {
    if keyboard_input.any_just_pressed([KeyCode::W, KeyCode::Up]) {
        let mut player = query
            .get_single_mut()
            .expect("There should only be one player");
        let new_player = player.go(CardinalDirection::North, &map);
        if let Some(new_player) = new_player {
            player_history.0.push(player.clone());
            *player = new_player;
        }
    } else if keyboard_input.any_just_pressed([KeyCode::D, KeyCode::Right]) {
        let mut player = query
            .get_single_mut()
            .expect("There should only be one player");
        let new_player = player.go(CardinalDirection::East, &map);
        if let Some(new_player) = new_player {
            player_history.0.push(player.clone());
            *player = new_player;
        }
    } else if keyboard_input.any_just_pressed([KeyCode::S, KeyCode::Down]) {
        let mut player = query
            .get_single_mut()
            .expect("There should only be one player");
        let new_player = player.go(CardinalDirection::South, &map);
        if let Some(new_player) = new_player {
            player_history.0.push(player.clone());
            *player = new_player;
        }
    } else if keyboard_input.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        let mut player = query
            .get_single_mut()
            .expect("There should only be one player");
        let new_player = player.go(CardinalDirection::West, &map);
        if let Some(new_player) = new_player {
            player_history.0.push(player.clone());
            *player = new_player;
        }
    } else if keyboard_input.just_pressed(KeyCode::Z)
        && keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
    {
        // undo the last move
        if let Some(old_player) = player_history.0.pop() {
            let mut player = query
                .get_single_mut()
                .expect("There should only be one player");
            *player = old_player;
        }
    }
}
