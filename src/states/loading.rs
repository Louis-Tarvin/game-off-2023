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
        .insert_resource(TransitionManager::Normal)
        .add_systems(OnEnter(GameState::Loading), setup_loading_screen)
        .add_systems(OnExit(GameState::Loading), cleanup);
    }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
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
    #[asset(path = "textures/1.png")]
    pub countdown_1: Handle<Image>,
    #[asset(path = "textures/2.png")]
    pub countdown_2: Handle<Image>,
    #[asset(path = "textures/3.png")]
    pub countdown_3: Handle<Image>,
    #[asset(path = "textures/4.png")]
    pub countdown_4: Handle<Image>,
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
    #[asset(path = "models/cave.glb#Scene0")]
    pub cave1: Handle<Scene>,
    #[asset(path = "models/cave.glb#Scene1")]
    pub cave2: Handle<Scene>,
    #[asset(path = "models/gem.glb#Scene0")]
    pub gem: Handle<Scene>,
}

#[derive(Component)]
pub struct LoadingUiRoot;

fn setup_loading_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                padding: UiRect::all(Val::Px(20.)),
                width: Val::Px(550.0),
                height: Val::Px(190.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            background_color: Color::BLACK.into(),
            ..Default::default()
        })
        .insert(LoadingUiRoot)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Loading...",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));
        });
}

fn cleanup(mut commands: Commands, root: Query<Entity, With<LoadingUiRoot>>) {
    commands.entity(root.single()).despawn_recursive();
}
