use bevy::prelude::*;

use crate::{
    player::Player,
    states::{level::DespawnOnTransition, loading::FontAssets, transition::TransitionManager},
};

use super::{constants::UI_YELLOW, keys::StaminaCosts, UiRoot};

#[derive(Component)]
pub struct FailureUIRoot;

pub fn setup_failure_help(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(DespawnOnTransition)
        .insert(FailureUIRoot)
        .insert(UiRoot)
        .insert(Name::new("Failure help UI"))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(550.0),
                        height: Val::Px(190.0),
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..Default::default()
                    },
                    background_color: UI_YELLOW.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Out of stamina!",
                        TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 45.0,
                            color: Color::WHITE,
                        },
                    ));
                    parent.spawn(TextBundle::from_section(
                        "Press ctrl-z to undo last move",
                        TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 45.0,
                            color: Color::WHITE,
                        },
                    ));
                    parent.spawn(TextBundle::from_section(
                        "or press 'r' to restart level.",
                        TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 45.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

pub fn check_if_no_valid_moves(
    mut root: Query<&mut Visibility, With<FailureUIRoot>>,
    stamina_costs: Res<StaminaCosts>,
    player: Query<&Player>,
    transition_manager: Res<TransitionManager>,
) {
    if let Ok(player) = player.get_single() {
        if let Ok(mut visibility) = root.get_single_mut() {
            let mut is_valid_move = false;
            if player.stamina != 0 {
                if let Some(val) = stamina_costs.north {
                    is_valid_move = val as u16 <= player.stamina;
                }
                if let Some(val) = stamina_costs.east {
                    is_valid_move = val as u16 <= player.stamina;
                }
                if let Some(val) = stamina_costs.south {
                    is_valid_move = val as u16 <= player.stamina;
                }
                if let Some(val) = stamina_costs.west {
                    is_valid_move = val as u16 <= player.stamina;
                }
            }
            if player.stamina >= 4 {
                is_valid_move = true;
            }

            if is_valid_move || !matches!(*transition_manager, TransitionManager::Normal) {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Visible;
            }
        }
    }
}
