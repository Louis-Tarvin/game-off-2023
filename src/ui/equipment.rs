use bevy::prelude::*;

use crate::{
    equipment::Inventory,
    states::{
        level::{DespawnOnTransition, LevelManager},
        loading::FontAssets,
    },
};

use super::{
    constants::{DARK_GREY, GREY, UI_YELLOW, UI_YELLOW_HOVER},
    UiRoot,
};

#[derive(Debug, Component)]
pub struct InventoryCounter(Equipment);

#[derive(Debug, Component)]
pub struct PickingUiRoot;
#[derive(Debug, Component)]
pub struct InfoUiRoot;

#[derive(Debug, Component)]
pub struct AddButton {
    pub equipment: Equipment,
    pub weight: u8,
}

#[derive(Debug, Component)]
pub struct SubtractButton {
    pub equipment: Equipment,
    pub weight: u8,
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
    pub weight: u8,
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
                        padding: UiRect::all(Val::Px(20.)),
                        margin: UiRect::all(Val::Px(10.)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
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
                    parent.spawn(TextBundle::from_section(
                        format!("weight: {}", equipment.weight),
                        TextStyle {
                            font: font.clone(),
                            font_size: 15.0,
                            color: Color::WHITE,
                        },
                    ));
                    parent.spawn(TextBundle::from_section(
                        equipment.description,
                        TextStyle {
                            font: font.clone(),
                            font_size: 15.0,
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
                            weight: equipment.weight,
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
                            weight: equipment.weight,
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
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.),
                width: Val::Percent(100.0),
                height: Val::Px(300.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        }).insert(PickingUiRoot)
        .insert(UiRoot)
        .insert(DespawnOnTransition)
        .with_children(|parent| {
            draw_equimpment_card(
                parent,
                font_assets.fira_sans.clone(),
                EquipmentInfo {
                    variant: Equipment::Ladder,
                    name: "Ladder".to_string(),
                    description: "Used to climb up two squares using less stamina. Can also be placed horizontally to cross gaps.".to_string(),
                    weight: 2,
                },
            );
            draw_equimpment_card(
                parent,
                font_assets.fira_sans.clone(),
                EquipmentInfo {
                    variant: Equipment::Rope,
                    name: "Rope".to_string(),
                    description: "Used to descend/ascend cliffs of any height using less stamina.".to_string(),
                    weight: 1,
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
    level_manager: Res<LevelManager>,
) {
    for (interaction, mut color, add) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if level_manager.get_current_level().weight_budget >= inventory.weight + add.weight
                {
                    inventory.weight += add.weight;
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
) {
    for (interaction, mut color, sub) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match sub.equipment {
                Equipment::Ladder => {
                    if inventory.ladder_count > 0 {
                        inventory.ladder_count -= 1;
                        inventory.weight -= sub.weight;
                    }
                }
                Equipment::Rope => {
                    if inventory.rope_count > 0 {
                        inventory.rope_count -= 1;
                        inventory.weight -= sub.weight;
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

fn draw_inventory_icon(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    equipment: Equipment,
    key: char,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(20.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // Key
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
            // Icon
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(40.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: UI_YELLOW.into(),
                ..Default::default()
            });
            // Count
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    border_color: DARK_GREY.into(),
                    background_color: GREY.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    let style = TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    };
                    parent
                        .spawn(TextBundle::from_sections([
                            TextSection::new("(", style.clone()),
                            TextSection::new("0", style.clone()),
                            TextSection::new(")", style),
                        ]))
                        .insert(InventoryCounter(equipment));
                });
        });
}

pub fn draw_inventory_icons(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.),
                width: Val::Percent(100.0),
                height: Val::Px(150.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(InfoUiRoot)
        .insert(UiRoot)
        .insert(DespawnOnTransition)
        .with_children(|parent| {
            draw_inventory_icon(
                parent,
                font_assets.fira_sans.clone(),
                Equipment::Ladder,
                '1',
            );
            draw_inventory_icon(parent, font_assets.fira_sans.clone(), Equipment::Rope, '2');
        });
}
