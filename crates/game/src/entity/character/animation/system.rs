
use bevy::{
    prelude::*,
    asset::{LoadContext, ron, AsyncReadExt, io::Reader, AssetLoader, BoxedFuture, LoadedAsset},
    utils::HashMap
};
use serde::Deserialize;


use crate::entity::character::movement::{CharacterMovementState, LookingAt};

use super::animation::{AnimationTimer, SpriteSheetAnimationsConfiguration, SpriteSheetConfiguration};
use thiserror::Error;

#[derive(Deserialize, Default, Asset, TypePath, Component, Clone)]
pub struct CharacterAnimationConfiguration {
    pub animations: SpriteSheetAnimationsConfiguration,
    pub sprite_sheet: SpriteSheetConfiguration,
}

#[derive(Default)]
pub struct CharacterAnimationStateHandle {
    pub handle: Handle<CharacterAnimationConfiguration>,
    pub loaded: bool,
    pub config: Option<CharacterAnimationConfiguration>,
    pub texture_loaded: bool,
}

#[derive(Default, Resource)]
pub struct CharacterAnimationConfigurationState(pub HashMap<String, CharacterAnimationStateHandle>);

impl CharacterAnimationConfigurationState {

	pub fn add_handler(&mut self, asset_server: &Res<AssetServer>, name: &str, path: String) {
        let lol_path = path.clone();
		self.0.insert(name.to_string(), CharacterAnimationStateHandle {
			handle: asset_server.load(lol_path),
			loaded: false,
			config: None,
			texture_loaded: false,
		});
	}
}

#[derive(Default)]
pub struct CharacterAnimationConfigurationLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CustomAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}


impl AssetLoader for CharacterAnimationConfigurationLoader {
    type Asset = CharacterAnimationConfiguration;
    type Settings = ();
    type Error = CustomAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = ron::de::from_bytes::<CharacterAnimationConfiguration>(&bytes)?;
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["animation.ron"]
    }
}



pub fn system_character_animation_config(
    mut state: ResMut<CharacterAnimationConfigurationState>,
    custom_assets: ResMut<Assets<CharacterAnimationConfiguration>>
) {
	for state in state.0.values_mut() {
		if !state.loaded {
			let v = custom_assets.get(&state.handle);
			if v.is_some() {
				state.loaded = true;
	            state.config = Some(v.unwrap().clone());
			}
		}
	}
}

pub fn react_character_animation(
    mut asset_events: EventReader<AssetEvent<CharacterAnimationConfiguration>>,
    mut commands: Commands,
) {
    for event in asset_events.read() {
        match event {
            AssetEvent::Modified { .. } => {
				println!("UPDATE");
            }
            _ => {}
        }
    }
}


fn validate_asset_loading(
    asset_server: &Res<AssetServer>,
	player_config_state: &mut CharacterAnimationStateHandle,
    texture_atlases_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    // TODO move to the setup directy
    /*
    if player_config_state.config.is_none() {
        return;
    } else if !player_config_state.texture_loaded {
        let config = player_config_state.config.as_ref().unwrap();
        let texture_handle = asset_server.load(config.sprite_sheet.path.as_str());
        let layout = TextureAtlasLayout::from_grid(
            config.sprite_sheet.tile_size, 
            config.sprite_sheet.columns, 
            config.sprite_sheet.rows,
            None,
            None
        );
        let texture_atlas_layout = texture_atlases_layout.add(layout);
        player_config_state.texture_loaded = true;
    }
    */
}

pub fn system_animation_character(
    mut q_player: Query<(&CharacterMovementState, &mut TextureAtlas, &mut AnimationTimer, &mut LookingAt, &mut Transform)>,

    mut player_config_state: ResMut<CharacterAnimationConfigurationState>,

    time: Res<Time>,
) {

    for (movement_state, mut atlas_sprite, mut timer, mut looking_at, mut transform) in q_player.iter_mut() {

        timer.timer.tick(time.delta());

	    let mut player_config_state = player_config_state.0.get_mut(timer.asset_type.as_str()).unwrap();

        //validate_asset_loading(&asset_server, &mut player_config_state, &mut texture_atlases);

        let config = player_config_state.config.as_ref();
        if config.is_none() { return; }
        let config = config.unwrap();

        //let handle_texture_atlas = player_config_state.handle_texture_atlas.as_ref().unwrap();

        //if handle.id != handle_texture_atlas.id {
        //    handle.id = handle_texture_atlas.id;
        //}


        if let Some(animation) = config.animations.0.get(&movement_state.state) {
            let state = movement_state.sub_state.clone() + movement_state.state.as_str();
            if !timer.current_state.eq(state.as_str()) {
                timer.current_state = state.clone();
                timer.index = 0;
                timer.timer = Timer::from_seconds(animation.playback_speed, TimerMode::Repeating);
            } else {
                if timer.timer.just_finished() {
                    timer.index += 1;
                    if timer.index >= animation.indexs.len() {
                        if animation.run_once {
                            timer.timer.pause();
                            continue;
                        }
                        timer.index = 0;
                    }
                }
            }
            atlas_sprite.index = timer.offset + animation.indexs[timer.index];
        }
    }
}

pub fn system_looking_at(
    mut q_character: Query<(&mut Transform, &mut LookingAt)>
) {
    for (mut transform, mut looking_at) in q_character.iter_mut() {
        let diff = if !looking_at.1 { (looking_at.0 - transform.translation.truncate()) } else {
            looking_at.0
        };
        let angle = diff.y.atan2(diff.x);

        transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
        transform.rotation = Quat::from_rotation_z(angle);
    }
}



pub fn setup_character_animation_config(
    mut state: ResMut<CharacterAnimationConfigurationState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
	state.add_handler(&asset_server, "player", "characters/player/player.animation.ron".to_string());
	state.add_handler(&asset_server, "zombie", "characters/zombie/zombie.animation.ron".to_string())
}

pub struct CharacterAnimationPlugin {}

impl Plugin for CharacterAnimationPlugin {
	fn build(&self, app: &mut App) {
        app
			.init_resource::<CharacterAnimationConfigurationState>()
            .init_asset::<CharacterAnimationConfiguration>()
            .init_asset_loader::<CharacterAnimationConfigurationLoader>()
            .add_systems(Startup, setup_character_animation_config)
            .add_systems(Update, (system_character_animation_config, system_animation_character, system_looking_at));
	}
}

