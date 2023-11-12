use bevy::prelude::*;

use crate::{equipment::Inventory, player::Player, states::loading::FontAssets};

use super::constants::{UI_YELLOW, UI_YELLOW_HOVER};

#[derive(Debug, Component)]
pub struct InventoryCounter(Equipment);

#[derive(Debug, Component)]
pub struct AddButton {
    pub equipment: Equipment,
    pub cost: u8,
}

#[derive(Debug, Component)]
pub struct SubtractButton {
    pub equipment: Equipment,
    pub cost: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum Equipment {
    Ladder,
    Rope,
}

struct EquipmentInfo {
    pub variant: Equipment,
    pub name: String,
    pub description: String,
    pub weight: f32,
    pub stamina_cost: u8,
}

fn draw_equimpment_card(parent: &mut ChildBuilder, font: Handle<Font>, equipment: EquipmentInfo) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(300.0),
                padding: UiRect::all(Val::Px(10.)),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // main body
            parent
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Px(200.0),
                        margin: UiRect::all(Val::Px(10.)),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    background_color: UI_YELLOW.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        equipment.name,
                        TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    ));
                });
            // button wrapper
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        flex_grow: 1.0,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    // - button
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(30.0),
                                height: Val::Px(30.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: UI_YELLOW.into(),
                            ..Default::default()
                        })
                        .insert(SubtractButton {
                            equipment: equipment.variant,
                            cost: equipment.stamina_cost,
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "-",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                    // have
                    let style = TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    };
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                height: Val::Px(30.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                flex_grow: 1.0,
                                ..Default::default()
                            },
                            background_color: UI_YELLOW.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_sections([
                                    TextSection::new("have: ", style.clone()),
                                    TextSection::new("0", style),
                                ]))
                                .insert(InventoryCounter(equipment.variant));
                        });
                    // + button
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(30.0),
                                height: Val::Px(30.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: UI_YELLOW.into(),
                            ..Default::default()
                        })
                        .insert(AddButton {
                            equipment: equipment.variant,
                            cost: equipment.stamina_cost,
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "+",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });
        });
}

pub fn draw_equimpment_cards(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(300.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            draw_equimpment_card(
                parent,
                font_assets.fira_sans.clone(),
                EquipmentInfo {
                    variant: Equipment::Ladder,
                    name: "Ladder".to_string(),
                    description: "Lorem ipsum".to_string(),
                    weight: 1.0,
                    stamina_cost: 5,
                },
            );
            draw_equimpment_card(
                parent,
                font_assets.fira_sans.clone(),
                EquipmentInfo {
                    variant: Equipment::Rope,
                    name: "Rope".to_string(),
                    description: "Lorem ipsum".to_string(),
                    weight: 1.0,
                    stamina_cost: 2,
                },
            );
        });
}

pub fn update_inventory_counters(
    inventory: Res<Inventory>,
    mut counters: Query<(&mut Text, &InventoryCounter)>,
) {
    for (mut text, counter) in counters.iter_mut() {
        match counter.0 {
            Equipment::Ladder => text.sections[1].value = format!("{}", inventory.ladder_count),
            Equipment::Rope => text.sections[1].value = format!("{}", inventory.rope_count),
        }
    }
}

pub fn handle_add_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &AddButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut inventory: ResMut<Inventory>,
    mut player: Query<&mut Player>,
) {
    let mut player = player
        .get_single_mut()
        .expect("There should only be one player");
    for (interaction, mut color, add) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(stamina) = player.stamina.checked_sub(add.cost.into()) {
                    player.stamina = stamina;
                    match add.equipment {
                        Equipment::Ladder => inventory.ladder_count += 1,
                        Equipment::Rope => inventory.rope_count += 1,
                    }
                }
            }
            Interaction::Hovered => {
                *color = UI_YELLOW_HOVER.into();
            }
            Interaction::None => {
                *color = UI_YELLOW.into();
            }
        }
    }
}

pub fn handle_subtract_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &SubtractButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut inventory: ResMut<Inventory>,
    mut player: Query<&mut Player>,
) {
    let mut player = player
        .get_single_mut()
        .expect("There should only be one player");
    for (interaction, mut color, sub) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match sub.equipment {
                Equipment::Ladder => {
                    if inventory.ladder_count > 0 {
                        inventory.ladder_count -= 1;
                        player.stamina += sub.cost as u16;
                    }
                }
                Equipment::Rope => {
                    if inventory.rope_count > 0 {
                        inventory.rope_count -= 1;
                        player.stamina += sub.cost as u16;
                    }
                }
            },
            Interaction::Hovered => {
                *color = UI_YELLOW_HOVER.into();
            }
            Interaction::None => {
                *color = UI_YELLOW.into();
            }
        }
    }
}
