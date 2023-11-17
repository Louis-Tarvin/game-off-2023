use bevy::prelude::States;

pub mod level;
pub mod loading;
pub mod menu;
pub mod transition;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Level,
    LevelTransition,
}
