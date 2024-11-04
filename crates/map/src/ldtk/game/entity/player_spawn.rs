use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::game::entity::map::player_spawn::PlayerSpawnComponent;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerSpawnBundle {
    player_spawn: PlayerSpawnComponent,
    #[sprite_sheet_bundle]
    sprite_sheet: LdtkSpriteSheetBundle,
}
