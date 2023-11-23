use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/bgm.ogg")]
    pub bgm: Handle<AudioSource>,
    #[asset(path = "audio/pop.ogg")]
    pub pop: Handle<AudioSource>,
    #[asset(path = "audio/step1.ogg")]
    pub step1: Handle<AudioSource>,
    #[asset(path = "audio/step2.ogg")]
    pub step2: Handle<AudioSource>,
    #[asset(path = "audio/step3.ogg")]
    pub step3: Handle<AudioSource>,
    #[asset(path = "audio/step4.ogg")]
    pub step4: Handle<AudioSource>,
    #[asset(path = "audio/woosh.ogg")]
    pub woosh: Handle<AudioSource>,
    #[asset(path = "audio/teleport.ogg")]
    pub teleport: Handle<AudioSource>,
    #[asset(path = "audio/pickup.ogg")]
    pub pickup: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct VolumeSettings {
    pub sfx_vol: f64,
    pub music_vol: f64,
}
impl Default for VolumeSettings {
    fn default() -> Self {
        Self {
            music_vol: 1.0,
            sfx_vol: 1.0,
        }
    }
}

impl VolumeSettings {
    pub fn toggle_sfx_vol(&mut self) {
        self.sfx_vol -= 0.1;
        if self.sfx_vol < 0.0 {
            self.sfx_vol = 1.0;
        }
    }
    pub fn toggle_music_vol(&mut self) {
        self.music_vol -= 0.1;
        if self.music_vol < 0.0 {
            self.music_vol = 1.0;
        }
    }
}

#[derive(Component, Resource, Default, Clone)]
pub struct MusicChannel;
#[derive(Component, Resource, Default, Clone)]
pub struct SoundChannel;
