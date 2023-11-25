use bevy::prelude::*;

use crate::{post_process::TransitionSettings, ui::UiRoot, util::cubic_ease_in_out};

use super::GameState;

#[derive(Debug, Default, Resource)]
pub enum TransitionManager {
    #[default]
    Normal,
    TransitioningOutReload(f32),
    TransitioningOut(f32),
    TransitioningIn(f32),
}

pub fn update_transition_manager(
    mut transition_manager: ResMut<TransitionManager>,
    mut transition_settings: Query<&mut TransitionSettings>,
    mut state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    match transition_manager.as_mut() {
        TransitionManager::Normal => {}
        TransitionManager::TransitioningOut(p) => {
            if *p >= 1.0 {
                state.set(GameState::LevelTransition);
                *transition_manager = TransitionManager::TransitioningIn(1.0);
            } else {
                let new_p = *p + time.delta_seconds();
                for mut settings in transition_settings.iter_mut() {
                    settings.progress = cubic_ease_in_out(new_p);
                }
                *transition_manager = TransitionManager::TransitioningOut(new_p);
            }
        }
        TransitionManager::TransitioningIn(p) => {
            let new_p = *p - time.delta_seconds();
            if new_p <= 0.0 {
                for mut settings in transition_settings.iter_mut() {
                    settings.progress = 0.0;
                }
                *transition_manager = TransitionManager::Normal;
            } else {
                for mut settings in transition_settings.iter_mut() {
                    settings.progress = cubic_ease_in_out(*p);
                }
                *transition_manager = TransitionManager::TransitioningIn(new_p);
            }
        }
        TransitionManager::TransitioningOutReload(p) => {
            if *p >= 1.0 {
                state.set(GameState::LevelReload);
                *transition_manager = TransitionManager::TransitioningIn(1.0);
            } else {
                let new_p = *p + time.delta_seconds();
                for mut settings in transition_settings.iter_mut() {
                    settings.progress = cubic_ease_in_out(new_p);
                }
                *transition_manager = TransitionManager::TransitioningOutReload(new_p);
            }
        }
    }
}

pub fn hide_ui_on_transition(
    mut ui_roots: Query<&mut Visibility, With<UiRoot>>,
    transition_manager: Res<TransitionManager>,
) {
    match *transition_manager {
        TransitionManager::Normal => {
            // for mut visibility in ui_roots.iter_mut() {
            // *visibility = Visibility::Visible;
            // }
        }
        TransitionManager::TransitioningOut(_) | TransitionManager::TransitioningOutReload(_) => {
            for mut visibility in ui_roots.iter_mut() {
                *visibility = Visibility::Hidden;
            }
        }
        _ => {}
    }
}
