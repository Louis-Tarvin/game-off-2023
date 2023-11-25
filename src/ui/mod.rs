use bevy::prelude::Component;

pub mod constants;
pub mod equipment;
pub mod failure;
pub mod keys;
pub mod stamina;

#[derive(Component)]
pub struct UiRoot;
