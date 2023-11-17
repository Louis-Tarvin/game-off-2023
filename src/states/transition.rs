use bevy::prelude::*;

use crate::post_process::TransitionSettings;

use super::{level::LevelManager, GameState};

#[derive(Debug, Default, Resource)]
pub enum TransitionManager {
    #[default]
    Normal,
    TransitioningOutMenu(f32),
    TransitioningOut(f32),
    TransitioningIn(f32),
}

fn cubic_ease_in_out(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let t = t * 2.0;

    if t < 1.0 {
        return 0.5 * t * t * t;
    }

    let t = t - 2.0;
    0.5 * (t * t * t + 2.0)
}

pub fn update_transition_manager(
    mut transition_manager: ResMut<TransitionManager>,
    mut transition_settings: Query<&mut TransitionSettings>,
    mut level_manager: ResMut<LevelManager>,
    mut state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    match transition_manager.as_mut() {
        TransitionManager::Normal => {}
        TransitionManager::TransitioningOut(p) => {
            let new_p = *p + time.delta_seconds();
            if new_p >= 1.0 {
                if level_manager.current + 1 < level_manager.levels.len() {
                    level_manager.current += 1;
                }
                state.set(GameState::LevelTransition);
                *transition_manager = TransitionManager::TransitioningIn(1.0);
            } else {
                for mut settings in transition_settings.iter_mut() {
                    settings.progress = cubic_ease_in_out(*p);
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
        TransitionManager::TransitioningOutMenu(p) => {
            let new_p = *p + time.delta_seconds();
            if new_p >= 1.0 {
                level_manager.current = 0;
                state.set(GameState::Level);
                *transition_manager = TransitionManager::TransitioningIn(1.0);
            } else {
                for mut settings in transition_settings.iter_mut() {
                    settings.progress = cubic_ease_in_out(*p);
                }
                *transition_manager = TransitionManager::TransitioningOutMenu(new_p);
            }
        }
    }
}
