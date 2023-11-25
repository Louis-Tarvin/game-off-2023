use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{audio::AudioAssets, states::transition::TransitionManager};

use super::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        info!("loading...");
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::MainMenu),
        )
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, ModelAssets>(GameState::Loading)
        .insert_resource(TransitionManager::Normal);
    }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
    #[asset(path = "textures/rune_circle.png")]
    pub rune_circle: Handle<Image>,
    #[asset(path = "textures/ladder.png")]
    pub ladder_icon: Handle<Image>,
    #[asset(path = "textures/rope.png")]
    pub rope_icon: Handle<Image>,
    #[asset(path = "textures/rune.png")]
    pub rune_icon: Handle<Image>,
    #[asset(path = "textures/potion.png")]
    pub potion_icon: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct ModelAssets {
    #[asset(path = "models/climber.glb#Scene0")]
    pub climber: Handle<Scene>,
    #[asset(path = "models/flag.glb#Scene0")]
    pub flag: Handle<Scene>,
    #[asset(path = "models/flag.glb#Animation0")]
    pub flag_animation: Handle<AnimationClip>,
    #[asset(path = "models/ladder.glb#Scene0")]
    pub ladder: Handle<Scene>,
    #[asset(path = "models/rope.glb#Scene0")]
    pub rope_top: Handle<Scene>,
    #[asset(path = "models/rope.glb#Scene1")]
    pub rope: Handle<Scene>,
    #[asset(path = "models/scale.glb#Scene0")]
    pub scale: Handle<Scene>,
}
