use bevy::prelude::*;

use crate::{
    scale::ScaleCounter,
    states::{level::DespawnOnTransition, loading::FontAssets},
};

use super::{constants::UI_YELLOW, UiRoot};

#[derive(Debug, Component)]
pub struct ScaleText;

pub fn setup_scale_count_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    scale_counter: Res<ScaleCounter>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(100.0),
                width: Val::Px(200.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: UI_YELLOW.into(),
            ..Default::default()
        })
        .insert(DespawnOnTransition)
        .insert(UiRoot)
        .insert(Name::new("Scale UI"))
        .with_children(|parent| {
            let style = TextStyle {
                font: font_assets.fira_sans.clone(),
                font_size: 20.0,
                color: Color::WHITE,
            };
            parent
                .spawn(TextBundle::from_sections([
                    TextSection::new("Scales collected: ", style.clone()),
                    TextSection::new(scale_counter.0.to_string(), style),
                ]))
                .insert(ScaleText);
        });
}

pub fn update_scale_count_ui(
    mut query: Query<&mut Text, With<ScaleText>>,
    scale_counter: Res<ScaleCounter>,
) {
    for mut text in query.iter_mut() {
        text.sections[1].value = format!("{}", scale_counter.0);
    }
}
