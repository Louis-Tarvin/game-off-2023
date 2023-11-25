use bevy::prelude::*;

use crate::{
    level_manager::LevelManager,
    player::Player,
    states::{level::DespawnOnTransition, loading::FontAssets},
    util::CardinalDirection,
};

use super::{constants::*, UiRoot};

#[derive(Debug, Default, Resource)]
pub struct StaminaCosts {
    pub north: Option<u8>,
    pub east: Option<u8>,
    pub south: Option<u8>,
    pub west: Option<u8>,
}

#[derive(Component)]
pub struct StaminaValue(CardinalDirection);

pub fn setup_keys_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_items: JustifyItems::Center,
                align_items: AlignItems::Center,
                left: Val::Px(0.0),
                top: Val::Px(130.0),
                width: Val::Px(200.0),
                height: Val::Px(100.0),
                ..Default::default()
            },
            background_color: UI_YELLOW.into(),
            ..Default::default()
        })
        .insert(DespawnOnTransition)
        .insert(UiRoot)
        .insert(Name::new("Keys UI"))
        .with_children(|parent| {
            let text_style = TextStyle {
                font: font_assets.fira_sans.clone(),
                font_size: 20.0,
                color: Color::WHITE,
            };
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        width: Val::Px(100.0),
                        height: Val::Px(100.0),
                        grid_template_columns: vec![
                            GridTrack::flex(1.0),
                            GridTrack::min_content(),
                            GridTrack::min_content(),
                            GridTrack::min_content(),
                            GridTrack::flex(1.0),
                        ],
                        grid_template_rows: vec![
                            GridTrack::auto(),
                            GridTrack::min_content(),
                            GridTrack::min_content(),
                            GridTrack::auto(),
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    // up value
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                align_self: AlignSelf::End,
                                justify_self: JustifySelf::Center,
                                grid_column: GridPlacement::start(3),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("", text_style.clone()))
                                .insert(StaminaValue(CardinalDirection::North));
                        });
                    // up key
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                grid_column: GridPlacement::start(3).set_span(3),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            draw_key(parent, font_assets.fira_sans.clone(), 'W');
                        });
                    // left value
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                align_self: AlignSelf::Center,
                                justify_self: JustifySelf::End,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("", text_style.clone()))
                                .insert(StaminaValue(CardinalDirection::West));
                        });
                    // left key
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            draw_key(parent, font_assets.fira_sans.clone(), 'A');
                        });
                    // down key
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                grid_column: GridPlacement::start(2),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            draw_key(parent, font_assets.fira_sans.clone(), 'S');
                        });
                    // right key
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                grid_column: GridPlacement::start(4),
                                grid_row: GridPlacement::start(3),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            draw_key(parent, font_assets.fira_sans.clone(), 'D');
                        });
                    // right value
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                align_self: AlignSelf::Center,
                                justify_self: JustifySelf::Start,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("", text_style.clone()))
                                .insert(StaminaValue(CardinalDirection::East));
                        });
                    // down value
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                align_self: AlignSelf::Start,
                                grid_column: GridPlacement::start(3),
                                grid_row: GridPlacement::start(4),
                                justify_self: JustifySelf::Center,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("", text_style.clone()))
                                .insert(StaminaValue(CardinalDirection::South));
                        });
                });
        });
}

pub fn draw_key(parent: &mut ChildBuilder, font: Handle<Font>, key: char) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(20.0),
                height: Val::Px(20.0),
                border: UiRect::bottom(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            border_color: DARK_GREY.into(),
            background_color: GREY.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                key,
                TextStyle {
                    font: font.clone(),
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            ));
        });
}

pub fn update_stamina_costs(
    player: Query<&Player, Changed<Player>>,
    level_manager: Res<LevelManager>,
    mut stamina_costs: ResMut<StaminaCosts>,
) {
    if let Ok(player) = player.get_single() {
        let map = &level_manager.get_current_level().map;
        stamina_costs.north = player
            .go(CardinalDirection::North, map)
            .map(|p| (player.stamina - p.stamina) as u8);
        stamina_costs.east = player
            .go(CardinalDirection::East, map)
            .map(|p| (player.stamina - p.stamina) as u8);
        stamina_costs.south = player
            .go(CardinalDirection::South, map)
            .map(|p| (player.stamina - p.stamina) as u8);
        stamina_costs.west = player
            .go(CardinalDirection::West, map)
            .map(|p| (player.stamina - p.stamina) as u8);
    }
}

pub fn update_stamina_values(
    stamina_costs: Res<StaminaCosts>,
    mut stamina_text_vals: Query<(&mut Text, &StaminaValue)>,
) {
    for (mut text, val) in stamina_text_vals.iter_mut() {
        match val.0 {
            CardinalDirection::North => {
                if let Some(val) = stamina_costs.north {
                    text.sections[0].value = format!("-{}", val);
                } else {
                    text.sections[0].value = "".to_string();
                };
            }
            CardinalDirection::East => {
                if let Some(val) = stamina_costs.east {
                    text.sections[0].value = format!("-{}", val);
                } else {
                    text.sections[0].value = "".to_string();
                };
            }
            CardinalDirection::South => {
                if let Some(val) = stamina_costs.south {
                    text.sections[0].value = format!("-{}", val);
                } else {
                    text.sections[0].value = "".to_string();
                };
            }
            CardinalDirection::West => {
                if let Some(val) = stamina_costs.west {
                    text.sections[0].value = format!("-{}", val);
                } else {
                    text.sections[0].value = "".to_string();
                };
            }
        }
    }
}
