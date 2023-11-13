use bevy::prelude::*;

use crate::states::GameState;

use self::{
    ladder::{handle_ladder_input, Ladder},
    rope::handle_rope_input,
};

pub mod ladder;
pub mod rope;

pub struct EquipmentPlugin;

impl Plugin for EquipmentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ladder>()
            .register_type::<Inventory>()
            .insert_resource(Inventory::default())
            .add_systems(
                Update,
                (handle_ladder_input, handle_rope_input).run_if(in_state(GameState::Level)),
            );
    }
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct Inventory {
    pub ladder_count: u8,
    pub rope_count: u8,
    pub weight: u8,
}
