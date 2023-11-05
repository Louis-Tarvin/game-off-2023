use bevy::prelude::States;

pub mod level;
pub mod loading;
pub mod menu;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Level,
}
