use bevy::prelude::*;

use crate::states::GameState;

use self::{
    ladder::{handle_ladder_input, Ladder},
    rewind::handle_rewind_input,
    rope::handle_rope_input,
};

pub mod ladder;
pub mod rewind;
pub mod rope;

pub struct EquipmentPlugin;

impl Plugin for EquipmentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ladder>()
            .register_type::<Inventory>()
            .insert_resource(Inventory::default())
            .add_systems(
                Update,
                (handle_ladder_input, handle_rope_input, handle_rewind_input)
                    .run_if(in_state(GameState::Level)),
            );
    }
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct Inventory {
    pub ladder_count: u8,
    pub rope_count: u8,
    pub potion_count: u8,
    pub rewind_count: u8,
    pub weight: u8,
}
