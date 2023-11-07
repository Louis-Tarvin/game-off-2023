use bevy::prelude::*;

use crate::{
    map::Map,
    states::{loading::ModelAssets, GameState},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}
impl CardinalDirection {
    fn reverse(&self) -> Self {
        match self {
            CardinalDirection::North => CardinalDirection::South,
            CardinalDirection::East => CardinalDirection::West,
            CardinalDirection::South => CardinalDirection::North,
            CardinalDirection::West => CardinalDirection::East,
        }
    }
}

impl From<CardinalDirection> for Vec3 {
    fn from(value: CardinalDirection) -> Self {
        match value {
            CardinalDirection::North => Vec3::Z,
            CardinalDirection::East => Vec3::NEG_X,
            CardinalDirection::South => Vec3::NEG_Z,
            CardinalDirection::West => Vec3::X,
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub enum PlayerState {
    Standing(CardinalDirection),
    Climbing(ClimbingState),
}

#[derive(Debug, Clone, Reflect)]
pub struct ClimbingState {
    direction: CardinalDirection,
    elevation: u8,
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
        }
    }

    pub fn go(&self, direction: CardinalDirection, heights: &Vec<Vec<u8>>) -> Option<Self> {
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
                if let PlayerState::Standing(_) = self.state {
                    if y == heights.len() - 1 {
                        return None;
                    }
                }
            }
            CardinalDirection::West => {
                if x == 0 {
                    return None;
                }
            }
        }

        let current_elevation = heights[y][x];
        let (new_x, new_y) = match direction {
            CardinalDirection::North => (x, y - 1),
            CardinalDirection::East => (x + 1, y),
            CardinalDirection::South => (x, y + 1),
            CardinalDirection::West => (x - 1, y),
        };
        match &self.state {
            PlayerState::Standing(_) if heights[new_y][new_x] == current_elevation => {
                self.stamina.checked_sub(1).map(|stamina| Self {
                    stamina,
                    grid_pos_x: new_x as u8,
                    grid_pos_y: new_y as u8,
                    state: PlayerState::Standing(direction),
                })
            }
            PlayerState::Standing(_) if heights[new_y][new_x] > current_elevation => Some(Self {
                stamina: self.stamina,
                grid_pos_x: self.grid_pos_x,
                grid_pos_y: self.grid_pos_y,
                state: PlayerState::Climbing(ClimbingState {
                    direction,
                    elevation: current_elevation + 1,
                }),
            }),
            PlayerState::Standing(_) => Some(Self {
                stamina: self.stamina,
                grid_pos_x: new_x as u8,
                grid_pos_y: new_y as u8,
                state: PlayerState::Climbing(ClimbingState {
                    direction: direction.reverse(),
                    elevation: current_elevation,
                }),
            }),
            PlayerState::Climbing(climb_state) => match direction {
                CardinalDirection::North => {
                    // TODO: fix cases when not facing north
                    if heights[y - 1][x] == climb_state.elevation {
                        // climb on top
                        self.stamina.checked_sub(2).map(|stamina| Self {
                            stamina,
                            grid_pos_x: self.grid_pos_x,
                            grid_pos_y: self.grid_pos_y - 1,
                            state: PlayerState::Standing(climb_state.direction),
                        })
                    } else {
                        // climb up
                        self.stamina.checked_sub(5).map(|stamina| Self {
                            stamina,
                            grid_pos_x: self.grid_pos_x,
                            grid_pos_y: self.grid_pos_y,
                            state: PlayerState::Climbing(ClimbingState {
                                direction,
                                elevation: climb_state.elevation + 1,
                            }),
                        })
                    }
                }
                CardinalDirection::East => {
                    // TODO: fix cases when not facing north
                    if heights[y][x + 1] >= climb_state.elevation
                        || heights[y - 1][x + 1] < climb_state.elevation
                    {
                        // no valid climb spot to the east
                        None
                    } else {
                        // move east
                        self.stamina.checked_sub(2).map(|stamina| Self {
                            stamina,
                            grid_pos_x: self.grid_pos_x + 1,
                            grid_pos_y: self.grid_pos_y,
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
                    // TODO: fix cases when not facing north
                    if heights[y][x - 1] >= climb_state.elevation
                        || heights[y - 1][x - 1] < climb_state.elevation
                    {
                        // no valid climb spot to the east
                        None
                    } else {
                        // move east
                        self.stamina.checked_sub(2).map(|stamina| Self {
                            stamina,
                            grid_pos_x: self.grid_pos_x - 1,
                            grid_pos_y: self.grid_pos_y,
                            state: PlayerState::Climbing(ClimbingState {
                                direction: climb_state.direction,
                                elevation: climb_state.elevation,
                            }),
                        })
                    }
                }
            },
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
        let new_player = player.go(CardinalDirection::North, &map.grid_heights);
        if let Some(new_player) = new_player {
            player_history.0.push(player.clone());
            *player = new_player;
        }
    } else if keyboard_input.any_just_pressed([KeyCode::D, KeyCode::Right]) {
        let mut player = query
            .get_single_mut()
            .expect("There should only be one player");
        let new_player = player.go(CardinalDirection::East, &map.grid_heights);
        if let Some(new_player) = new_player {
            player_history.0.push(player.clone());
            *player = new_player;
        }
    } else if keyboard_input.any_just_pressed([KeyCode::S, KeyCode::Down]) {
        let mut player = query
            .get_single_mut()
            .expect("There should only be one player");
        let new_player = player.go(CardinalDirection::South, &map.grid_heights);
        if let Some(new_player) = new_player {
            player_history.0.push(player.clone());
            *player = new_player;
        }
    } else if keyboard_input.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        let mut player = query
            .get_single_mut()
            .expect("There should only be one player");
        let new_player = player.go(CardinalDirection::West, &map.grid_heights);
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
