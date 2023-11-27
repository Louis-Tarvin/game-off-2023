use bevy::prelude::*;

use crate::{level_manager::LevelManager, scale::ScaleCounter, states::loading::FontAssets};

use super::{
    constants::{SKY_BLUE, UI_YELLOW},
    UiRoot,
};

#[derive(Component)]
pub struct EndUIRoot;

pub fn setup_end_screen(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    scale_count: Res<ScaleCounter>,
    level_manager: Res<LevelManager>,
) {
    // Count the total number of scales across all levels
    let mut number_of_scales = 0;
    for level in level_manager.levels.iter() {
        if level.map.scale_pos.is_some() {
            number_of_scales += 1;
        }
    }
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: SKY_BLUE.into(),
            ..Default::default()
        })
        .insert(EndUIRoot)
        .insert(UiRoot)
        .insert(Name::new("End screen UI"))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(600.0),
                        height: Val::Px(190.0),
                        justify_content: JustifyContent::SpaceAround,
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
                        "You've climbed every mountain! Good job!",
                        TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 45.0,
                            color: Color::WHITE,
                        },
                    ));
                    parent.spawn(TextBundle::from_section(
                        format!("Collected {}/{} scales.", scale_count.0, number_of_scales),
                        TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 35.0,
                            color: Color::WHITE,
                        },
                    ));
                    let message = if scale_count.0 == number_of_scales {
                        "You got them all. Impressive!"
                    } else if scale_count.0 < 3 {
                        "Oh well. There's always next time..."
                    } else {
                        "Hey, not bad."
                    };
                    parent.spawn(TextBundle::from_section(
                        message,
                        TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 25.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}
