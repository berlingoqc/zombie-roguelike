use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::{game::entity::{door::DoorBundle, window::WindowBundle}, map_const};


pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_ldtk_entity::<WindowBundle>(map_const::ENTITY_WINDOW_LOCATION)
        .register_ldtk_entity::<DoorBundle>(map_const::ENTITY_DOOR_LOCATION);
    }
}

pub struct LdtkRoguePlugin;


impl Plugin for LdtkRoguePlugin {
   fn build(&self, app: &mut App) {
        app
        .add_plugins(LdtkPlugin)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: false,
            },
            ..default()
        })
        .add_plugins(EntityPlugin);
   } 
}