use bevy::prelude::*;

use crate::{map::Map, states::level::Level};

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct LevelManager {
    pub levels: Vec<Level>,
    pub current: usize,
}
impl LevelManager {
    pub fn get_current_level(&self) -> &Level {
        &self.levels[self.current]
    }
    pub fn get_current_map_mut(&mut self) -> &mut Map {
        &mut self.levels[self.current].map
    }
}

pub fn init_level_manager(mut commands: Commands) {
    commands.insert_resource(LevelManager {
        current: 0,
        levels: vec![
            // Introducing rewind rune
            Level {
                map: Map::new(
                    vec![
                        vec![5, 5, 5, 6, 7, 6, 5],
                        vec![4, 4, 4, 4, 4, 4, 3],
                        vec![2, 2, 1, 2, 3, 2, 1],
                    ],
                    vec![
                        vec![false, false, false, false, false, false, false],
                        vec![false, false, false, false, false, false, false],
                        vec![false, true, false, false, false, false, false],
                    ],
                    (1, 2),
                    (5, 0),
                    Some((2, 2)),
                ),
                stamina_budget: 8,
                weight_budget: 3,
                ladder_unlocked: true,
                rope_unlocked: true,
                potion_unlocked: true,
                rewind_unlocked: true,
            },
            Level {
                map: Map::new(
                    vec![
                        vec![2, 2, 5, 7, 7, 5],
                        vec![2, 3, 5, 5, 3, 3],
                        vec![2, 1, 1, 1, 2, 2],
                    ],
                    vec![
                        vec![true, true, true, true, true, true],
                        vec![true, true, true, true, true, true],
                        vec![true, true, true, true, true, true],
                    ],
                    (3, 2),
                    (4, 0),
                    Some((1, 0)),
                ),
                stamina_budget: 10,
                weight_budget: 4,
                ladder_unlocked: true,
                rope_unlocked: true,
                potion_unlocked: true,
                rewind_unlocked: true,
            },
            Level {
                map: Map::new(
                    vec![vec![6, 7, 7], vec![4, 5, 4], vec![4, 4, 2], vec![1, 1, 1]],
                    vec![
                        vec![true, true, true],
                        vec![true, true, true],
                        vec![true, true, true],
                        vec![true, true, true],
                    ],
                    (1, 3),
                    (2, 0),
                    None,
                ),
                stamina_budget: 16,
                weight_budget: 2,
                ladder_unlocked: true,
                rope_unlocked: true,
                potion_unlocked: true,
                rewind_unlocked: true,
            },
            // Intro level
            Level {
                map: Map::new(
                    vec![
                        vec![7, 8, 8, 9, 8, 7],
                        vec![6, 6, 6, 6, 5, 6],
                        vec![6, 5, 5, 5, 5, 6],
                        vec![4, 3, 3, 3, 3, 4],
                        vec![1, 1, 1, 1, 1, 2],
                    ],
                    vec![
                        vec![false, false, false, false, false, false],
                        vec![false, false, false, false, false, false],
                        vec![false, true, false, false, false, false],
                        vec![false, false, false, false, true, false],
                        vec![true, true, true, true, true, false],
                    ],
                    (0, 4),
                    (4, 1),
                    None,
                ),
                stamina_budget: 27,
                weight_budget: 0,
                ladder_unlocked: false,
                rope_unlocked: false,
                potion_unlocked: false,
                rewind_unlocked: false,
            },
            // Introducing ladders
            Level {
                map: Map::new(
                    vec![
                        vec![4, 5, 6, 6, 5, 5],
                        vec![3, 3, 2, 3, 3, 3],
                        vec![1, 1, 1, 2, 2, 1],
                    ],
                    vec![
                        vec![false, false, false, false, true, true],
                        vec![false, false, false, false, false, false],
                        vec![false, false, false, false, false, false],
                    ],
                    (0, 2),
                    (4, 0),
                    Some((5, 0)),
                ),
                stamina_budget: 14,
                weight_budget: 4,
                ladder_unlocked: true,
                rope_unlocked: false,
                potion_unlocked: false,
                rewind_unlocked: false,
            },
            Level {
                map: Map::new(
                    vec![
                        vec![4, 6, 4, 6, 7, 7],
                        vec![4, 5, 6, 6, 5, 5],
                        vec![3, 1, 1, 3, 3, 3],
                    ],
                    vec![
                        vec![false, false, false, false, false, false],
                        vec![true, true, true, true, false, false],
                        vec![false, false, false, false, false, false],
                    ],
                    (0, 2),
                    (5, 1),
                    None,
                ),
                stamina_budget: 11,
                weight_budget: 2,
                ladder_unlocked: true,
                rope_unlocked: false,
                potion_unlocked: false,
                rewind_unlocked: false,
            },
            Level {
                map: Map::new(
                    vec![
                        vec![4, 6, 4, 6, 6, 4],
                        vec![4, 6, 2, 6, 5, 5],
                        vec![3, 3, 1, 3, 3, 2],
                    ],
                    vec![
                        vec![false, false, false, false, false, false],
                        vec![false, false, false, false, false, false],
                        vec![false, false, false, false, false, false],
                    ],
                    (0, 2),
                    (5, 1),
                    None,
                ),
                stamina_budget: 8,
                weight_budget: 4,
                ladder_unlocked: true,
                rope_unlocked: false,
                potion_unlocked: false,
                rewind_unlocked: false,
            },
            // Introducing rope
            Level {
                map: Map::new(
                    vec![
                        vec![6, 6, 6, 6, 5, 5],
                        vec![4, 4, 3, 4, 3, 2],
                        vec![2, 3, 2, 4, 2, 2],
                    ],
                    vec![
                        vec![false, false, false, false, false, false],
                        vec![false, false, false, false, false, false],
                        vec![false, false, false, false, false, false],
                    ],
                    (0, 0),
                    (5, 2),
                    None,
                ),
                stamina_budget: 9,
                weight_budget: 2,
                ladder_unlocked: false,
                rope_unlocked: true,
                potion_unlocked: false,
                rewind_unlocked: false,
            },
            // Ladder plus rope
            Level {
                map: Map::new(
                    vec![
                        vec![6, 1, 6, 6, 5, 4],
                        vec![1, 6, 6, 2, 4, 4],
                        vec![6, 2, 2, 2, 3, 2],
                        vec![2, 2, 1, 2, 2, 2],
                    ],
                    vec![
                        vec![false, false, false, false, true, false],
                        vec![false, false, false, false, false, false],
                        vec![true, false, false, false, false, false],
                        vec![false, false, false, false, false, false],
                    ],
                    (0, 0),
                    (5, 0),
                    Some((2, 3)),
                ),
                stamina_budget: 15,
                weight_budget: 5,
                ladder_unlocked: true,
                rope_unlocked: true,
                potion_unlocked: true,
                rewind_unlocked: false,
            },
            // Reusing ladder
            Level {
                map: Map::new(
                    vec![
                        vec![7, 7, 7, 6],
                        vec![6, 5, 5, 5],
                        vec![3, 3, 3, 3],
                        vec![2, 1, 1, 1],
                    ],
                    vec![
                        vec![true, true, true, true],
                        vec![true, true, true, true],
                        vec![true, true, true, true],
                        vec![true, true, true, true],
                    ],
                    (1, 3),
                    (2, 0),
                    Some((0, 0)),
                ),
                stamina_budget: 19,
                weight_budget: 3,
                ladder_unlocked: true,
                rope_unlocked: true,
                potion_unlocked: false,
                rewind_unlocked: false,
            },
            Level {
                map: Map::new(
                    vec![
                        vec![5, 6, 6, 5, 6, 4],
                        vec![2, 3, 3, 2, 3, 3],
                        vec![1, 3, 1, 2, 3, 1],
                    ],
                    vec![
                        vec![true, true, true, true, true, true],
                        vec![true, true, true, true, true, true],
                        vec![true, true, true, true, true, true],
                    ],
                    (0, 2),
                    (5, 0),
                    None,
                ),
                stamina_budget: 30,
                weight_budget: 5,
                ladder_unlocked: true,
                rope_unlocked: true,
                potion_unlocked: false,
                rewind_unlocked: false,
            },
        ],
    })
}
