
use bevy::{prelude::*, utils::HashMap};
use serde::Deserialize;

#[derive(Deserialize, Default, Clone)]
pub struct SpriteSheetConfiguration {
	pub name: String,
	pub path: String,
	pub tile_size: Vec2,
	pub columns: usize,
	pub rows: usize
}

#[derive(Deserialize, Default, Clone)]
pub struct SpriteSheetAnimationConfiguration {
	pub sprite_sheet_name: String,
	pub state_name: String,
	pub indexs: Vec<usize>,
	pub playback_speed: f32,
	pub run_once: bool,
	//pub no_offset: bool,
}

#[derive(Deserialize, Default, Clone)]
pub struct SpriteSheetAnimationsConfiguration(pub HashMap<String, SpriteSheetAnimationConfiguration>);

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub index: usize,
    pub offset: usize,
    pub asset_type: String,
    pub current_state: String,
}