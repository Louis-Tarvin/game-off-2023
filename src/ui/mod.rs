use bevy::prelude::*;

use crate::{
    equipment::Inventory,
    scale::ScaleCounter,
    states::{
        transition::{hide_ui_on_transition, update_transition_manager, TransitionManager},
        GameState,
    },
};

use self::{
    end::setup_end_screen,
    equipment::{
        draw_equimpment_cards, draw_inventory_icons, handle_add_buttons, handle_subtract_buttons,
        update_inventory_counters, update_weight_text,
    },
    failure::{check_if_no_valid_moves, setup_failure_help},
    keys::{setup_keys_ui, update_stamina_costs, update_stamina_values},
    scale::{setup_scale_count_ui, update_scale_count_ui},
    stamina::{setup_stamina_ui, update_stamina_ui},
};

pub mod constants;
pub mod end;
pub mod equipment;
pub mod failure;
pub mod keys;
pub mod scale;
pub mod stamina;

#[derive(Component)]
pub struct UiRoot;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Level),
            (
                setup_stamina_ui,
                setup_keys_ui,
                setup_failure_help,
                setup_scale_count_ui,
                draw_equimpment_cards,
                draw_inventory_icons,
            ),
        )
        .add_systems(
            Update,
            update_transition_manager
                .run_if(in_state(GameState::MainMenu).or_else(in_state(GameState::Level))),
        )
        .add_systems(
            Update,
            hide_ui_on_transition.run_if(resource_changed::<TransitionManager>()),
        )
        .add_systems(
            Update,
            (
                update_stamina_ui,
                (
                    update_stamina_costs,
                    (update_stamina_values, check_if_no_valid_moves),
                )
                    .chain(),
                handle_add_buttons,
                handle_subtract_buttons,
                update_inventory_counters.run_if(resource_changed::<Inventory>()),
                update_weight_text.run_if(resource_changed::<Inventory>()),
                update_scale_count_ui.run_if(resource_changed::<ScaleCounter>()),
            )
                .run_if(in_state(GameState::Level)),
        )
        .add_systems(OnEnter(GameState::End), setup_end_screen);
    }
}
