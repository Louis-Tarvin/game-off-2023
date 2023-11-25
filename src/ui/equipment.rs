use bevy::prelude::*;

use crate::{
    equipment::Inventory,
    level_manager::LevelManager,
    player::Player,
    states::{
        level::DespawnOnTransition,
        loading::{FontAssets, TextureAssets},
    },
};

use super::{
    constants::{DARK_GREY, GREY, UI_YELLOW, UI_YELLOW_HOVER},
    keys::draw_key,
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
    Potion,
    Rewind,
}

struct EquipmentInfo {
    pub variant: Equipment,
    pub name: String,
    pub description: String,
    pub weight: u8,
}

fn draw_equimpment_card(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    equipment: EquipmentInfo,
    texture: Handle<Image>,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(400.0),
                height: Val::Px(160.0),
                padding: UiRect::all(Val::Px(10.)),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // main body
            parent
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Px(150.0),
                        flex_grow: 1.0,
                        padding: UiRect::all(Val::Px(5.)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: UI_YELLOW.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // left column
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(40.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                equipment.name,
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ));
                            parent.spawn(ImageBundle {
                                image: UiImage {
                                    texture,
                                    ..Default::default()
                                },
                                style: Style {
                                    width: Val::Px(80.0),
                                    height: Val::Px(80.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                            parent.spawn(TextBundle::from_section(
                                format!("weight: {}", equipment.weight),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                    // right column
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(60.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                equipment.description,
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });
            // button wrapper
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        margin: UiRect::left(Val::Px(5.0)),
                        width: Val::Px(60.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    // - button
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(60.0),
                                height: Val::Px(50.0),
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
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                height: Val::Px(50.0),
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
                                width: Val::Px(60.0),
                                height: Val::Px(50.0),
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

#[derive(Component)]
pub struct WeightText;

pub fn draw_equimpment_cards(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    level_manager: Res<LevelManager>,
    texture_assets: Res<TextureAssets>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(0.),
                width: Val::Px(400.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        }).insert(PickingUiRoot)
        .insert(UiRoot)
        .insert(DespawnOnTransition)
        .insert(Name::new("Equipment Cards UI"))
        .with_children(|parent| {
            let level = level_manager.get_current_level();
            if level.ladder_unlocked || level.rope_unlocked || level.potion_unlocked || level.rewind_unlocked {
                // Draw weight budget
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(380.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    background_color: UI_YELLOW.into(),
                    ..Default::default()
                }).with_children(|parent| {
                    let style = TextStyle {
                        font: font_assets.fira_sans.clone(),
                        font_size: 35.0,
                        color: Color::WHITE
                    };
                    parent.spawn(TextBundle::from_sections([
                        TextSection::new("Current weight: ", style.clone()),
                        TextSection::new("0", style.clone()),
                        TextSection::new(format!("/{}", level.weight_budget), style),
                    ]))
                    .insert(WeightText);
                });
            }
            if level.ladder_unlocked {
                draw_equimpment_card(
                    parent,
                    font_assets.fira_sans.clone(),
                    EquipmentInfo {
                        variant: Equipment::Ladder,
                        name: "Ladder".to_string(),
                        description: "Used to climb up two squares using less stamina. Can also be placed horizontally to cross gaps.".to_string(),
                        weight: 2,
                    },
                    texture_assets.ladder_icon.clone()
                );
            }
            if level.rope_unlocked {
                draw_equimpment_card(
                    parent,
                    font_assets.fira_sans.clone(),
                    EquipmentInfo {
                        variant: Equipment::Rope,
                        name: "Rope".to_string(),
                        description: "Used to descend/ascend cliffs of any height using less stamina.".to_string(),
                        weight: 1,
                    },
                    texture_assets.rope_icon.clone()
                );
            }
            if level.rewind_unlocked {
                draw_equimpment_card(
                    parent,
                    font_assets.fira_sans.clone(),
                    EquipmentInfo {
                        variant: Equipment::Rewind,
                        name: "Rune of Rewind".to_string(),
                        description: "Once placed you have 5 turns until you are teleported back to it, reclaiming any lost stamina (placed equipment remains)".to_string(),
                        weight: 1,
                    },
                    texture_assets.rune_icon.clone()
                );
            }
            if level.potion_unlocked {
                draw_equimpment_card(
                    parent,
                    font_assets.fira_sans.clone(),
                    EquipmentInfo {
                        variant: Equipment::Potion,
                        name: "Stamina Potion".to_string(),
                        description: "A flask of green liquid. Gives a small stamina boost".to_string(),
                        weight: 2
                    },
                    texture_assets.potion_icon.clone()
                );
            }
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
            Equipment::Potion => text.sections[1].value = format!("{}", inventory.potion_count),
            Equipment::Rewind => text.sections[1].value = format!("{}", inventory.rewind_count),
        }
    }
}

pub fn update_weight_text(mut text: Query<&mut Text, With<WeightText>>, inventory: Res<Inventory>) {
    for mut text in text.iter_mut() {
        text.sections[1].value = inventory.weight.to_string();
    }
}

pub fn handle_add_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &AddButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut inventory: ResMut<Inventory>,
    level_manager: Res<LevelManager>,
    mut player: Query<&mut Player>,
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
                        Equipment::Potion => {
                            inventory.potion_count += 1;
                            player.get_single_mut().unwrap().stamina += 2;
                        }
                        Equipment::Rewind => inventory.rewind_count += 1,
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
                Equipment::Potion => {
                    if inventory.potion_count > 0 {
                        inventory.potion_count -= 1;
                        inventory.weight -= sub.weight;
                        player.get_single_mut().unwrap().stamina -= 2;
                    }
                }
                Equipment::Rewind => {
                    if inventory.rewind_count > 0 {
                        inventory.rewind_count -= 1;
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
    texture: Handle<Image>,
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
            draw_key(parent, font.clone(), key);
            // Icon
            parent.spawn(ImageBundle {
                image: UiImage {
                    texture,
                    ..Default::default()
                },
                style: Style {
                    width: Val::Px(40.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
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
                            TextSection::new(" ", style.clone()),
                            TextSection::new("0", style.clone()),
                            TextSection::new(" ", style),
                        ]))
                        .insert(InventoryCounter(equipment));
                });
        });
}

pub fn draw_inventory_icons(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    level_manager: Res<LevelManager>,
    texture_assets: Res<TextureAssets>,
) {
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
        .insert(Name::new("Inventory icons UI"))
        .with_children(|parent| {
            let level = level_manager.get_current_level();
            if level.ladder_unlocked {
                draw_inventory_icon(
                    parent,
                    font_assets.fira_sans.clone(),
                    Equipment::Ladder,
                    '1',
                    texture_assets.ladder_icon.clone(),
                );
            }
            if level.rope_unlocked {
                draw_inventory_icon(
                    parent,
                    font_assets.fira_sans.clone(),
                    Equipment::Rope,
                    '2',
                    texture_assets.rope_icon.clone(),
                );
            }
            if level.rewind_unlocked {
                draw_inventory_icon(
                    parent,
                    font_assets.fira_sans.clone(),
                    Equipment::Rewind,
                    '3',
                    texture_assets.rune_icon.clone(),
                );
            }
        });
}
