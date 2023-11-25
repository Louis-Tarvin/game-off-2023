use bevy::prelude::*;

use crate::{
    player::Player,
    states::{level::DespawnOnTransition, loading::FontAssets},
};

use super::{constants::UI_YELLOW, UiRoot};

#[derive(Debug, Component)]
pub struct StaminaText;

pub fn setup_stamina_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Px(200.0),
                height: Val::Px(100.0),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: UI_YELLOW.into(),
            ..Default::default()
        })
        .insert(DespawnOnTransition)
        .insert(UiRoot)
        .insert(Name::new("Stamina UI"))
        .with_children(|parent| {
            let style = TextStyle {
                font: font_assets.fira_sans.clone(),
                font_size: 40.0,
                color: Color::WHITE,
            };
            parent
                .spawn(
                    TextBundle::from_sections([
                        TextSection::new("Stamina: ", style.clone()),
                        TextSection::from_style(style),
                    ])
                    .with_style(Style {
                        top: Val::Px(50.0),
                        ..Default::default()
                    }),
                )
                .insert(StaminaText);
        });
}

pub fn update_stamina_ui(mut query: Query<&mut Text, With<StaminaText>>, player: Query<&Player>) {
    for mut text in query.iter_mut() {
        if let Ok(player) = player.get_single() {
            text.sections[1].value = format!("{}", player.stamina);
        }
    }
}
