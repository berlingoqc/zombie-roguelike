use bevy::prelude::*;

use super::entity::character::animation::system::CharacterAnimationPlugin;

pub struct BaseZombieGamePlugin;

impl Plugin for BaseZombieGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CharacterAnimationPlugin {});
    }
}
